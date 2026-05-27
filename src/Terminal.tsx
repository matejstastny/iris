import { useEffect, useRef } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';

function TerminalView() {
    const containerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const term = new Terminal();
        const fitAddon = new FitAddon();

        term.loadAddon(fitAddon);
        term.open(containerRef.current!);
        fitAddon.fit();

        term.write('Hello from xterm.js!\r\n');

        return () => term.dispose();
    }, []);

    return <div ref={containerRef} style={{ height: '100%', width: '100%' }} />;
}

export default TerminalView;
