use portable_pty::{native_pty_system, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

struct ShellState {
    writer: Mutex<Option<Box<dyn Write + Send>>>,
    master: Mutex<Option<Box<dyn MasterPty + Send>>>,
}

#[tauri::command]
fn start_shell(state: State<ShellState>, app: AppHandle, cols: u16, rows: u16) {
    if state.writer.lock().unwrap().is_some() {
        return;
    }

    let pty_system = native_pty_system();
    let pty_pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("failed to open pty");

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let mut cmd = CommandBuilder::new(shell);
    cmd.env("TERM", "xterm-256color");
    pty_pair
        .slave
        .spawn_command(cmd)
        .expect("failed to spawn shell");

    let writer = pty_pair.master.take_writer().expect("failed to get writer");
    let mut reader = pty_pair
        .master
        .try_clone_reader()
        .expect("failed to get reader");

    *state.writer.lock().unwrap() = Some(writer);
    *state.master.lock().unwrap() = Some(pty_pair.master);

    std::thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let output = String::from_utf8_lossy(&buf[..n]).to_string();
                    app.emit("shell-output", output).ok();
                }
            }
        }
    });
}

#[tauri::command]
fn write_to_shell(state: State<ShellState>, data: String) {
    if let Some(writer) = state.writer.lock().unwrap().as_mut() {
        writer.write_all(data.as_bytes()).ok();
    }
}

#[tauri::command]
fn resize_shell(state: State<ShellState>, cols: u16, rows: u16) {
    if let Some(master) = state.master.lock().unwrap().as_mut() {
        master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .ok();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ShellState {
            writer: Mutex::new(None),
            master: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            start_shell,
            write_to_shell,
            resize_shell
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
