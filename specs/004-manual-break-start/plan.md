# Implementation Plan: Manual Break Start

**Branch**: `004-manual-break-start` | **Date**: 2026-01-03 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/004-manual-break-start/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Change timer state transition behavior so that work session completion rings a chime and displays break time (5:00) without automatically starting the countdown. Users must manually press Start to begin the break session. Break completion similarly displays work time (25:00) without auto-starting. This gives users control over when they actually begin breaks and work sessions.

## Technical Context

**Language/Version**: Rust 1.92  
**Primary Dependencies**: tauri 2.9  
**Storage**: N/A  
**Testing**: cargo test  
**Target Platform**: macOS and Linux Desktop (`ubuntu-latest`)  
**Project Type**: single  
**Performance Goals**: NO concrete plans  
**Constraints**: NO concrete plans  
**Scale/Scope**: one user per one app

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

✅ **Code Quality**: Existing Rust standards in place (cargo fmt, clippy). Changes will follow established patterns in timer.rs module.

✅ **Testing Standards**: Existing test framework at `src-tauri/src/timer/tests.rs`. Will add tests for new states and transitions.

✅ **User Experience Consistency**: Change maintains existing control scheme (Start, Pause, Resume, Clear buttons). Adds manual control which improves consistency by treating work and break sessions uniformly.

✅ **Performance Requirements**: No performance concerns - same timer mechanics, just different state transitions. No additional resources required.

✅ **Simplicity**: Avoids over-engineering. Uses existing Status enum pattern, adds logical "ready" states to distinguish from "running". No new dependencies or complex patterns needed.

**Gate Status**: ✅ PASS - No violations. All principles satisfied.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# This single project
src-tauri/
├── capabilities/
├── icons/
└── src/

src/
└── main.rs
```

**Structure Decision**: Single desktop application with Tauri backend (Rust) and frontend (TypeScript). Timer logic resides in `src-tauri/src/timer.rs` with IPC commands in `src-tauri/src/lib.rs`. Frontend polling and UI updates in `src/main.ts`. No additional projects or modules needed for this feature.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

N/A - No constitution violations identified.

---

## Phase 0: Research & Design Decisions ✅ COMPLETE

**Artifacts**: [research.md](research.md)

**Key Decisions**:



1. **State Representation**: Add `WorkReady` and `BreakReady` status values to Status enum
2. **State Transitions**: Completions transition to Ready states (not auto-start)
3. **Start Command**: Make phase-aware (starts work or break based on current status)
4. **Chime Timing**: Trigger on completionFlag when transitioning to Ready states



**Unknowns Resolved**:

- ✅ How to distinguish "ready" from "running"? → New Status enum values
- ✅ When to play chime? → On transition to Ready states via completionFlag
- ✅ How to support starting breaks? → Phase-aware start() method

**Dependencies**: No new dependencies required

---


## Phase 1: Data Model & Contracts ✅ COMPLETE


**Artifacts**:


- [data-model.md](data-model.md) - State machine, entities, validation rules
- [contracts/ipc-commands.md](contracts/ipc-commands.md) - Tauri IPC API contract

- [quickstart.md](quickstart.md) - Developer onboarding guide


**Data Model**:

- **TimerState**: phase, status, remaining_secs, duration_secs, completion_flag, state_label

- **Status Enum**: WorkReady, BreakReady, Running, Paused, Complete
- **Phase Enum**: Work, Break (unchanged)

**API Contracts**:

- `get_state()` - Returns current TimerState
- `start_timer()` - Phase-aware start (work or break)
- `pause_timer()` - Pause running countdown
- `resume_timer()` - Resume paused countdown
- `clear_timer()` - Reset to WorkReady

**State Machine**: Defined with clear transitions between Ready and Running states

**Agent Context**: No updates needed - tech stack already documented in copilot-instructions.md

---

## Constitution Check - Post Design ✅ RE-VERIFIED

**Code Quality**: ✅ Design follows existing patterns, no new complexity introduced

**Testing Standards**: ✅ Test strategy defined in research.md, existing framework sufficient

**User Experience Consistency**: ✅ Manual control improves UX by giving users agency over transitions

**Performance Requirements**: ✅ No performance impact - same polling, same calculations

**Simplicity**: ✅ Minimal changes to Status enum and handle_completion(), no over-engineering

**Final Gate Status**: ✅ PASS - Design meets all constitution principles

---

## Phase 2: Task Breakdown

**Status**: NOT STARTED (use `/speckit.tasks` command)

**Artifacts**: tasks.md (to be generated)

**Purpose**: Break down implementation into concrete, executable tasks with acceptance criteria

---

## Next Steps

1. ✅ Specification complete ([spec.md](spec.md))
2. ✅ Research complete ([research.md](research.md))
3. ✅ Data model complete ([data-model.md](data-model.md))
4. ✅ Contracts complete ([contracts/ipc-commands.md](contracts/ipc-commands.md))
5. ✅ Quickstart guide complete ([quickstart.md](quickstart.md))
6. ⏭️ **NEXT**: Run `/speckit.tasks` to generate task breakdown
7. ⏭️ **THEN**: Begin implementation following tasks.md

---

## Summary

This implementation plan defines a clean, minimal approach to adding manual break control:

- **Backend**: Expand Status enum, modify handle_completion() to not auto-start, update start() to be phase-aware
- **Frontend**: Update status type, adjust button logic, simplify chime detection
- **Testing**: Add tests for new Ready states and transitions
- **Zero new dependencies**: Uses existing Rust std, serde, and Tauri patterns

The design maintains simplicity while giving users control over their Pomodoro workflow timing.
