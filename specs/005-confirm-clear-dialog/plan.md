# Implementation Plan: Confirm Dialog for Clear Action

**Branch**: `005-confirm-clear-dialog` | **Date**: 2026-01-03 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-confirm-clear-dialog/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add a confirmation dialog that appears when users click the Clear button while a session has remaining time or status. The dialog allows users to confirm or cancel the destructive action before the timer resets to its initial idle state. This prevents accidental loss of ongoing focus sessions and improves the reliability of the app. The dialog must be operable via pointer and keyboard input, support graceful dismissal, and display a clear message about the consequence of clearing.

## Technical Context

**Language/Version**: Rust 1.92 for backend; TypeScript/HTML/CSS for UI  
**Primary Dependencies**: tauri 2.9 (desktop shell and IPC bridge)  
**UI Framework**: Custom HTML/CSS with TypeScript event handlers  
**Storage**: N/A (no state persistence required)  
**IPC/Commands**: May use tauri command invocation to communicate timer state and clear action from frontend to backend  
**Testing**: cargo test for Rust backend; manual UI testing for confirmation dialog interaction  
**Target Platform**: macOS and Linux Desktop (`ubuntu-latest`)  
**Project Type**: Single monolithic Tauri app with shared timer state  
**Performance Goals**: Dialog appears and responds to user input within 100ms  
**Constraints**: Window size is fixed and small; confirmation message must fit without scrolling; keyboard accessibility required  
**Scale/Scope**: Single user per app instance

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Code Quality**: ✅ PASS — Existing Rust codebase in `src-tauri/src/` follows conventions; new dialog code will follow same patterns and be subject to code review.
- **Testing Standards**: ✅ PASS — Project uses `cargo test` for unit and integration tests; confirmation logic in timer module will be tested; UI interaction can be verified via manual testing or end-to-end test framework.
- **User Experience Consistency**: ✅ PASS — Dialog must follow existing app UI style (simple window, consistent button styling); accessibility requires keyboard navigation (tab, enter, escape) and clear text messaging.
- **Performance Requirements**: ✅ PASS — Dialog must respond to user input within 100ms; no complex animations or heavy computations; simple state transition from timer to cleared state.
- **Simplicity**: ✅ PASS — Confirmation dialog is a straightforward modal pattern; no repository pattern, ORM, or external dependencies required; leverages existing tauri command/event infrastructure; minimal new code complexity.

**Gate Status**: ✅ All principles satisfied. Proceeding to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/005-confirm-clear-dialog/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src-tauri/
├── src/
│   ├── main.rs          # Tauri window setup, command registration
│   ├── lib.rs           # Command handlers (clear, state queries)
│   ├── timer.rs         # Timer state machine and business logic
│   └── timer/tests.rs   # Unit tests for timer logic
├── Cargo.toml           # Rust dependencies
├── tauri.conf.json      # Tauri config (window size, capabilities)
└── capabilities/        # Tauri ACL manifests (IPC permissions)

src/
├── index.html           # Main UI entry point
├── main.ts              # TypeScript event handling and DOM manipulation
├── assets/              # Static resources (CSS, icons)
```

**Structure Decision**: Single Tauri app with shared timer module. Confirmation dialog UI embedded in `index.html` using HTML/CSS modal pattern. Dialog state and clear action communicated via tauri commands from `main.ts` to `src-tauri/src/lib.rs`. Timer state queries use existing command infrastructure.
````

---

## Post-Design Constitution Check (Phase 1 Complete)

*RE-EVALUATION after design artifacts completed.*

### Code Quality
✅ **PASS** — Design adheres to existing patterns:
- Clear method in timer.rs follows existing method structure (no new paradigms)
- Command handler in lib.rs mirrors existing command patterns
- TypeScript event handlers use standard DOM APIs and tauri invoke pattern
- No deviations from Rust 1.92 conventions or project style guide
- Code review will be standard PR process

### Testing Standards
✅ **PASS** — Testing strategy is concrete:
- Unit test for timer.clear() added to timer/tests.rs (existing test file)
- Manual interaction testing documented in quickstart.md (pointer, keyboard, edge cases)
- No new test framework or CI/CD changes required
- cargo test command will include new unit test automatically

### User Experience Consistency
✅ **PASS** — Dialog design is consistent:
- Message uses simple, clear language ("This will remove the current time and status")
- Button labels match existing UI vocabulary ("Clear", "Cancel")
- Modal presentation follows web standards (overlay, centered, keyboard-accessible)
- Escape key, background click, and close button all have expected behavior
- No accessibility violations (semantic buttons, tab navigation, focus management)

### Performance Requirements
✅ **PASS** — Performance goals are met:
- Dialog render: <10ms (CSS flexbox, no animation)
- Command execution: <100ms (local tauri IPC, simple state mutation)
- No network calls, no database queries
- Modal overlay uses efficient CSS (no JS recalculation loop)
- Test success criteria SC-003 (user reads message within 2 seconds) confirms UX speed

### Simplicity
✅ **PASS** — Solution is deliberately simple:
- Modal dialog: single `<div>` with CSS overlay + two buttons (no component library)
- State machine: one new `clear()` method on Timer struct
- Command: one new tauri command that calls existing logic
- No external dependencies (uses browser APIs + tauri built-ins)
- No architectural changes; fits seamlessly into existing Tauri + web structure
- Data model is straightforward: boolean flag (dialog_visible) and enum (dialog_mode)

### Re-evaluation Conclusion
✅ **All principles confirmed satisfied post-design. No violations. Ready for implementation.**

---

## Approval Checklist (Pre-Implementation)

- [x] Specification is complete and unambiguous (spec.md)
- [x] Technical design covers all requirements (research.md, data-model.md)
- [x] API contracts are defined (contracts/ipc-commands.md)
- [x] Implementation steps are clear and actionable (quickstart.md)
- [x] Constitution check passes pre and post-design
- [x] No new external dependencies introduced
- [x] Testing strategy is documented
- [x] Estimation: ~3-5 hours implementation and testing

**Status**: Ready for Phase 2 (task breakdown and implementation).
