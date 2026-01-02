# pomodoro_timer Development Guidelines

## Objective

This app supports people work with the Pomodoro Technique: 25 minutes work and 5 minutes break.

- Simple GUI so that users can understand usage soon
- Small window size not to disturb users

## Active Technologies

- Rust 1.92 + tauri 2.9 (desktop shell, command IPC)

## Language

- In chat, speak in English if there are no special instructions.
- All deliverables MUST be written in English.
  - deliverables examples: code comments, spec-kit outputs.

## Commands

Before running Rust (`cargo`) commands, you should move to `src-tauri`.

- `cargo tauri dev` for running app
- `cargo test` for tests
- `cargo fmt` for formatter

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

- All code must follow styles described in `.editorconfig` file
- Rust 1.92: Follow standard conventions

