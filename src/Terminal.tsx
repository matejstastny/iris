import { useEffect, useRef } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import '@xterm/xterm/css/xterm.css';

function TerminalView() {
    const containerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const term = new Terminal();
        const fitAddon = new FitAddon();

        term.loadAddon(fitAddon);
        term.open(containerRef.current!);
        fitAddon.fit();

        invoke('start_shell');

        const unlisten = listen<string>('shell-output', (event) => {
            term.write(event.payload);
        });

        term.onData((data) => {
            invoke('write_to_shell', { data });
        });

        return () => {
            unlisten.then((f) => f());
            term.dispose();
        };
    }, []);

    return <div ref={containerRef} style={{ height: '100%', width: '100%' }} />;
}

export default TerminalView;
