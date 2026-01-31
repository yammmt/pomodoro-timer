# Implementation Plan: Show Elapsed Time After Session Completion

**Branch**: `009-show-elapsed-time` | **Date**: 2026-01-31 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/009-show-elapsed-time/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add overtime display to the Pomodoro timer. When a work or break session completes (reaches 0:00), instead of just staying at 0:00, the timer continues tracking elapsed time in an "overtime" format with red color and minus prefix (e.g., "-02:13"). This provides immediate visual feedback that the session has ended and time is passing without action. Maximum overtime is capped at 59:59.

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

- **Code Quality**: ✅ PASS - Will follow Rust and TypeScript conventions, maintain existing code review process
- **Testing Standards**: ✅ PASS - Will add unit tests using existing cargo test framework for overtime logic
- **User Experience Consistency**: ✅ PASS - Overtime display uses existing timer display area, maintains UI consistency with color change only
- **Performance Requirements**: ✅ PASS - No performance concerns; continues existing 1Hz polling pattern
- **Simplicity**: ✅ PASS - Extends existing timer state machine without adding complexity; no new dependencies

**Result**: All constitution checks PASS. No violations to justify.

## Project Structure

### Documentation (this feature)

```text
specs/009-show-elapsed-time/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── timer-state.md   # Extended TimerState interface with overtime fields
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
└── main.ts
```

**Structure Decision**: Single project structure maintained. Changes localized to:

- Backend: `src-tauri/src/timer.rs` - Add overtime tracking to TimerService
- Backend: `src-tauri/src/timer/tests.rs` - Add overtime test cases
- Frontend: `src/main.ts` - Add overtime display logic and styling
- Frontend: `src/index.html` - Add overtime styles + modal

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations. All constitution checks passed.

---

## Phase 0: Research ✅ COMPLETE

**Output**: [research.md](research.md)

**Key Decisions**:

- Overtime tracking via `completed_at: Option<Instant>` timestamp
- Display representation via `overtime_secs: Option<u32>` in TimerState
- CSS-based styling (red color via `.overtime` class)
- 59:59 cap using `min(elapsed, 3599)` clamp

**Technologies**: No new dependencies required (uses std::time::Instant, existing serde, CSS)

---

## Phase 1: Design & Contracts ✅ COMPLETE

**Outputs**:

- [data-model.md](data-model.md) - Extended TimerService and TimerState entities
- [contracts/timer-state.md](contracts/timer-state.md) - IPC contract specification

- [quickstart.md](quickstart.md) - Developer implementation guide

**Design Highlights**:

- Backward-compatible extension (optional `overtime_secs` field)
- No new Status enum value; overtime is sub-state of Complete
- Frontend-backend contract maintained via Tauri IPC
- State transitions preserve existing state machine behavior

---

## Constitution Re-Check (Post-Design)

*Re-evaluating after completing design phase:*

- **Code Quality**: ✅ PASS - Design follows Rust idioms (Option types, const caps), TypeScript best practices
- **Testing Standards**: ✅ PASS - Unit tests defined in quickstart, covers edge cases (0s, cap, clear)
- **User Experience Consistency**: ✅ PASS - Reuses existing timer display element, only adds CSS class
- **Performance Requirements**: ✅ PASS - No additional polling, calculation is O(1), negligible overhead
- **Simplicity**: ✅ PASS - Two fields added (completed_at, overtime_secs), no architectural changes

**Final Result**: All constitution checks PASS after design. Ready for Phase 2 (task breakdown).

---

## Implementation Readiness

**Phase 0 & 1 artifacts complete**:

- ✅ Research decisions documented
- ✅ Data model designed
- ✅ API contract specified
- ✅ Quickstart guide written
- ✅ Agent context updated
- ✅ Constitution compliance verified

**Ready for**: `/speckit.tasks` command to generate task breakdown and implementation checklist

---

## Conclusion

The overtime feature extends the existing timer with minimal, non-breaking changes:

1. **Backend**: Add `completed_at` timestamp and `overtime_secs` calculation
2. **Frontend**: Check for `overtimeSecs` and apply conditional formatting
3. **Testing**: Unit tests for overtime logic, manual UI verification

No new dependencies, no architectural changes, full backward compatibility maintained.

**Branch**: `009-show-elapsed-time`  
**Next Command**: `/speckit.tasks` to begin implementation phase
