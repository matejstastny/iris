use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

struct ShellState {
    writer: Mutex<Option<Box<dyn Write + Send>>>,
}

#[tauri::command]
fn start_shell(state: State<ShellState>, app: AppHandle) {
    if state.writer.lock().unwrap().is_some() {
        return;
    }
    let pty_system = native_pty_system();

    let pty_pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("failed to open pty");

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

    let cmd = CommandBuilder::new(shell);
    pty_pair
        .slave
        .spawn_command(cmd)
        .expect("failed to spawn shell");

    let writer = pty_pair.master.take_writer().expect("failed to get writer");
    *state.writer.lock().unwrap() = Some(writer);

    let mut reader = pty_pair
        .master
        .try_clone_reader()
        .expect("failed to get reader");

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ShellState {
            writer: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![start_shell, write_to_shell])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
