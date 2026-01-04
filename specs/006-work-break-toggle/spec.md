# Feature Specification: Work/Break Mode Toggle

**Feature Branch**: `006-work-break-toggle`  
**Created**: 2026-01-04  
**Status**: Draft  
**Input**: User description: "Add work/break mode selection to the top of the app window. If the user clicks the \"Work\" button, then timer is set as work session, and if the user clicks the \"Break\" button, then timer is set as break session. These two buttons are aligned to left, in a row. Active session button has emphasized CSS style (for example, button color is changed to light color and added line edge). If the user clicks the other session button, the current session time is paused, not cleared."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Set a Work Session (Priority: P1)

A focused user wants to start or prepare a work session by selecting the Work mode from the top of the window so the timer is configured for work without hunting through settings.

**Why this priority**: Establishing a work session is the primary flow and must be fast and obvious to keep users in their task rhythm.

**Independent Test**: From an idle timer, the user can click Work and see the work session ready with correct duration and visual highlight without affecting break time.

**Acceptance Scenarios**:

1. **Given** the timer is idle with no active session, **When** the user clicks the Work button, **Then** the work session becomes active with its standard duration visible and the Work button shows its emphasized state.
2. **Given** the break session was last used and remains paused, **When** the user clicks the Work button, **Then** the break session remains paused with its remaining time preserved while the work session becomes active and ready to start.

---

### User Story 2 - Switch to Break Without Losing Progress (Priority: P2)

While working, a user wants to switch to a break session mid-countdown so the work timer pauses, its remaining time is preserved, and the break session is prepared to run.

**Why this priority**: Smooth switching keeps the Pomodoro rhythm without accidental time loss when changing context.

**Independent Test**: With a running work session, clicking Break pauses work time, preserves remaining seconds, and shows the break session active with its standard duration ready to start.

**Acceptance Scenarios**:

1. **Given** a work session is running, **When** the user clicks the Break button, **Then** the work timer pauses with remaining time stored, the break session becomes active with its standard duration displayed, and the Break button shows emphasized styling.
2. **Given** the break session is running, **When** the user clicks the Work button, **Then** the break timer pauses with remaining time stored, the work session becomes active with its standard duration displayed, and the Work button shows emphasized styling.

---

### User Story 3 - Recognize Active Mode at a Glance (Priority: P3)

Any user wants to instantly see which session is active via clear styling so they avoid starting the wrong timer.

**Why this priority**: Visual clarity reduces mistakes and supports rapid interaction.

**Independent Test**: With either session selected, users can reliably identify the active session solely by button styling without reading labels or starting the timer.

**Acceptance Scenarios**:

1. **Given** the work session is active, **When** the user views the top buttons, **Then** the Work button appears emphasized and the Break button appears inactive.
2. **Given** the break session is active, **When** the user views the top buttons, **Then** the Break button appears emphasized and the Work button appears inactive.

---

### Edge Cases

- Clicking the already active session button does not reset or change the remaining time and keeps the timer state unchanged.
- Rapidly switching between Work and Break preserves each session's remaining time without dropping seconds or starting a countdown automatically.
- When a session reaches zero, switching to the other session still pauses the completed session state and prepares the other session with its standard duration.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The app MUST display two left-aligned buttons in a single row at the top of the window labeled "Work" and "Break".
- **FR-002**: Selecting "Work" MUST set the active session to Work, display the standard work duration, and keep the timer paused until the user explicitly starts it.
- **FR-003**: Selecting "Break" MUST set the active session to Break, display the standard break duration, and keep the timer paused until the user explicitly starts it.
- **FR-004**: Switching sessions while a timer is running MUST pause the currently active session and preserve its remaining time for resumption when reselected.
- **FR-005**: The active session's button MUST show an emphasized style distinct from the inactive button (e.g., lighter color and edge indication) and update immediately upon selection.
- **FR-006**: Switching sessions MUST NOT clear or reset the paused session's remaining time unless the user performs a separate reset or clear action.

### Key Entities *(include if feature involves data)*

- **Session mode**: Represents the current session type (Work or Break), its standard duration, and its remaining time when paused.
- **Timer state**: Represents whether the timer is running, paused, or idle for the current session and tracks transitions when switching between modes.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can switch between Work and Break modes in no more than two clicks, with the active style updating within one second in usability tests.
- **SC-002**: Remaining time for the previously active session is preserved after a mode switch in 100% of observed test switches.
- **SC-003**: In usability observation, at least 95% of participants correctly identify the active session based on button styling without starting the timer.
- **SC-004**: Mode switching does not auto-start or reset timers, as confirmed across all acceptance scenarios during testing.

### Assumptions

- Standard durations follow the current Pomodoro defaults (work 25 minutes, break 5 minutes) when preparing a session after a switch.
- Switching sessions loads the standard duration for the newly selected session while retaining the paused remaining time for the previously active one.
- Clicking the already active session button leaves timer state and remaining time unchanged.
