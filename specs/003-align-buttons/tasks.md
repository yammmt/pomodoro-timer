# Tasks: Align Buttons in Single Row

**Input**: Design documents from `/specs/003-align-buttons/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, quickstart.md ✓

**Scope**: Single CSS property modification to prevent button wrapping in Pomodoro timer GUI
**Estimated Duration**: 10 minutes total
**MVP Status**: This is the complete feature - no phasing needed

---

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1)
- Tasks include exact file paths

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Feature branch and initial structure preparation

- [x] T001 Verify feature branch `003-align-buttons` is checked out
- [x] T002 Review implementation plan in `specs/003-align-buttons/plan.md`

---

## Phase 2: User Story 1 - All controls visible in single row (Priority: P1) 🎯 MVP

**Goal**: Modify CSS to ensure all four action buttons display in a single horizontal row without wrapping

**Independent Test**: Launch app at default window, verify all four buttons (Start, Pause, Resume, Clear) visible on one line with no text truncation, and layout remains stable during interaction

### Implementation for User Story 1

- [x] T003 [US1] Modify `.controls` CSS rule in `src/index.html` (line ~37-43): change `flex-wrap: wrap;` to `flex-wrap: nowrap;`
- [x] T004 [US1] Verify button layout at default window dimensions (1024x768 or larger)
- [x] T005 [US1] Verify all button text is fully visible and not truncated
- [x] T006 [US1] Test layout stability: hover over buttons and verify no reflow
- [x] T007 [US1] Test layout stability: click Start button to enable Pause/Resume and verify no layout shift
- [x] T008 [US1] Test layout stability: click Clear button to disable buttons and verify no layout shift
- [x] T009 [US1] Verify buttons remain aligned horizontally on window resize (down to ~400px width)

**Acceptance Criteria**:

- ✅ All four buttons displayed on single line
- ✅ No button text truncated
- ✅ Layout stable during all interactions
- ✅ All spec requirements met (FR-001 through FR-005)
- ✅ All success criteria met (SC-001 through SC-004)

**Checkpoint**: Feature complete and ready for code review

---

## Phase 3: Polish & Documentation

**Purpose**: Documentation and cleanup

- [x] T010 [P] Commit changes with message: `feat: align buttons in single row` (per conventional commits)
- [x] T011 Update CHANGELOG with feature completion (if maintained)
- [x] T012 Create pull request for feature branch `003-align-buttons`
- [x] T013 Verify CI/CD pipeline passes (if applicable)

---

## Dependencies & Execution Order

### Overall Sequence

1. **T001-T002** (Setup): Read and verify prerequisites - **must complete first**
2. **T003-T009** (Implementation): Modify CSS and test - **proceeds after setup**
   - T003 is the code change
   - T004-T009 are testing tasks (can run concurrently with visual verification)
3. **T010-T013** (Polish): Commit and PR - **proceeds after implementation**

### Parallel Opportunities

- T004-T009 can all be performed in parallel during testing phase - all are visual verifications of the same code change
- No functional dependencies between testing tasks

### Single Developer Path

For a single developer, the sequence is straightforward:

```
T001 → T002 → T003 → (T004-T009 in parallel via visual testing) → T010 → T011 → T012 → T013
```

Estimated timeline:

- T001-T002: 1 minute (setup)
- T003: 1 minute (code change)
- T004-T009: 5 minutes (visual testing)
- T010-T013: 3 minutes (commit and PR)
- **Total: ~10 minutes**

---

## Implementation Notes

### File to Modify

**File**: `src/index.html`  
**Location**: `.controls` CSS rule (approximately line 37-43)

**Current CSS**:

```css
.controls {
    display: flex;
    gap: 10px;
    justify-content: center;
    flex-wrap: wrap;
}
```

**Required Change**:

```css
.controls {
    display: flex;
    gap: 10px;
    justify-content: center;
    flex-wrap: nowrap;
}
```

### Testing Strategy

Visual regression testing is appropriate for this feature:

- Launch the Tauri app with `cargo tauri dev`
- Visually verify each acceptance criterion
- No automated unit tests needed for pure CSS layout changes
- No backend/Rust code changes

### No Other Files Modified

- `src/main.ts` - unchanged
- `src-tauri/src/` - unchanged
- All other files - unchanged

---

## Success Criteria Mapping

| Task | Spec Requirement | Success Criterion |
|------|------------------|-------------------|
| T003 | FR-001, FR-002 | SC-001, SC-003 |
| T004-T005 | FR-003 | SC-002 |
| T006-T009 | FR-004, FR-005 | SC-003, SC-004 |

---

## Rollback Plan

If any issues arise:

1. Revert T003 by changing `flex-wrap: nowrap;` back to `flex-wrap: wrap;`
2. Verify layout returns to original state
3. Consider alternative CSS approaches from `research.md`

---

## Definition of Done

A task is complete when:

- [x] Code changes implemented as specified
- [x] All acceptance scenarios verified
- [x] No regressions in existing functionality
- [x] Commit message follows conventional commits format
- [x] Ready for code review
