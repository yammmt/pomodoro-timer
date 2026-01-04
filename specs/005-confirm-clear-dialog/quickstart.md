# Quickstart: Confirm Dialog for Clear Action

**Duration**: ~30–45 minutes to verify  
**Tech Stack**: Rust 1.92, Tauri 2.9, TypeScript, HTML/CSS  
**Scope**: Confirm dialog when clearing an active timer; skip dialog when idle

---

## What Changed (implemented)

- Frontend: Added modal overlay, dialog markup, and confirm/cancel handlers in [src/index.html](../../src/index.html) and [src/main.ts](../../src/main.ts)
- Behavior: Dialog appears only when status is `running` or `paused`; idle/ready clears bypass dialog and Clear stays disabled
- Tests: Added Rust unit test for idle clear idempotency in [src-tauri/src/timer/tests.rs](../../src-tauri/src/timer/tests.rs)
- Backend: Reused existing `clear_timer` command; no contract changes

---

## How to Validate

### 1) Format, lint, and tests

```bash
cd src-tauri
cargo fmt
cargo clippy
cargo test
```

### 2) Run the app

```bash
cd src-tauri
cargo tauri dev
```

### 3) Manual scenarios (keyboard + pointer)

- Running → Clear → dialog shows; Confirm resets to idle (00:00, ready state)
- Running → Clear → Cancel keeps time/status unchanged
- Running → Clear → press Escape closes dialog without clearing
- Running → Clear → click outside dialog closes without clearing
- Idle/ready → Clear performs immediate clear (no dialog) and Clear stays disabled
- Focus: dialog opens with Confirm focused; Tab cycles, Enter activates focused button

---

## File Map

- UI markup/styles: [src/index.html](../../src/index.html)
- UI logic: [src/main.ts](../../src/main.ts)
- Timer test: [src-tauri/src/timer/tests.rs](../../src-tauri/src/timer/tests.rs)

---

## Notes

- No new commands or dependencies added; `clear_timer` already existed
- Modal styling is inline in index.html to keep bundle minimal
- Timer polling stops on clear to avoid redundant updates

---

## Done / To Verify

- [x] Commands above executed locally
- [x] Manual scenarios exercised
- [x] No regressions in timer start/pause/resume
