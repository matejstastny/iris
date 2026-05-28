import { useEffect, useRef } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import '@xterm/xterm/css/xterm.css';

function TerminalView() {
    const containerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const term = new Terminal({
            fontFamily: '"DankMono Nerd Font Mono", monospace',
            fontSize: 16,
            theme: {
                background: '#11111B',
            },
        });

        const fitAddon = new FitAddon();

        term.loadAddon(fitAddon);
        term.open(containerRef.current!);
        fitAddon.fit();

        const { cols, rows } = term;
        const unlistenPromise = listen<string>('shell-output', (event) => {
            term.write(event.payload);
        });

        unlistenPromise.then(() => invoke('start_shell', { cols, rows }));

        const handleResize = () => fitAddon.fit();
        window.addEventListener('resize', handleResize);

        term.onResize(({ cols, rows }) => {
            invoke('resize_shell', { cols, rows });
        });

        term.onData((data) => {
            invoke('write_to_shell', { data });
        });

        return () => {
            window.removeEventListener('resize', handleResize);
            unlistenPromise.then((f) => f());
            term.dispose();
        };
    }, []);

    return (
        <div className="terminal" ref={containerRef} style={{ height: '100%', width: '100%' }} />
    );
}

export default TerminalView;
