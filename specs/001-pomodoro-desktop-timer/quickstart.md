# Quickstart — Pomodoro Desktop Timer

## Prerequisites
- Rust 1.92 (with cargo)
- Node 18+ and npm (for Tauri frontend tooling)
- Tauri CLI 2.x (`cargo install tauri-cli` or `npm install -g @tauri-apps/cli`)
- macOS or Linux desktop environment

## Setup
1) Install dependencies
```sh
npm install
```
2) Build and run in dev
```sh
cargo tauri dev
```
3) Run backend tests
```sh
cargo test
```

## Expected behavior
- App opens a single window showing remaining time, Start, Pause/Resume, and Clear controls.
- Starting from idle begins a 25:00 work countdown; completion auto-starts a 5:00 break.
- Pause/Resume toggles without losing remaining time; Clear resets to 25:00 work-ready state.
- Visual plus audible cue fires on work and break completion; overlapping starts are blocked.
