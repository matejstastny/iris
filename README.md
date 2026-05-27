<!-- LOGO, sourced from https://github.com/ghostty-org/ghostty (thank you!) -->
<h1>
<p align="center">
  <img src="./assets/iris.png" alt="Logo" width="128">
  <br>Iris
</h1>
  <p align="center">
    a terminal that knows where you are
    <br />
    The CLI is powerful. But windows pile up, state gets lost, and nothing talks to each other.
    <code>iris</code> fixes that, without the chaos of tmux, without the friction of a full IDE.
    <br />
    <a href="#the-idea">About</a>
    ·
    <a href="https://matejstastny.github.io/iris/download">Download</a>
    ·
    <a href="https://matejstastny.github.io/iris/docs">Documentation</a>
    ·
    <a href="HACKING.md">Developing</a>
  </p>
</p>

## `what it is`

iris is a terminal emulator built on [Tauri](https://tauri.app) & React. It is a terminal with a little less text UI and more GUI and native integration with things like SSH, Docker, Git or AI agents (yeah I know, I know, its what people want though). All of that with having a lot of things running and a persistant window state.

## `the idea`

I live in multiple projects at once - robotics, school, personal. Switching context between them should take a keypress, not a ritual. Yes I did try `tmux`. I have a pretty deep config for it, but it just does not work as I imagined it.

- **splits** so you can see things side by side (duh)
- **workspaces** that separate your projects without overlap or confusion, with color/icon separation
- **tabs for things that matter** - git, docker, ssh, not just more terminals. All of them can be named.
- **state that survives** - close the app, reopen, everything is where you left it

## `built with`

| layer    | tech                        |
| -------- | --------------------------- |
| shell    | `xterm.js` & `portable-pty` |
| backend  | Rust + Tauri                |
| frontend | React + TypeScript          |
| styling  | CSS (no framework)          |

## `integrations` _(planned)_

- `git` - branch, status, log at a glance
- `docker` - containers, images, logs without leaving the terminal
- `ssh` - saved hosts, persistent sessions
- `scripting` - lua config and scripting

## `status`

**WIP.** This is also a learning project — Rust, Tauri, and modern web without leaning on AI.
Expect rough edges. Progress is intentional and slow.
