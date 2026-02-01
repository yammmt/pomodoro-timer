# Feature Specification: Show Elapsed Time After Session Completion

**Feature Branch**: `009-show-elapsed-time`  
**Created**: 2026-01-31  
**Status**: Draft  
**Input**: User description: "Show elapsed time after a session has completed. For example, if work session has completed 2 minutes and 13 seconds ago, the app shows '-02:13' in red color."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Work Session Overtime Display (Priority: P1)

When a user's work session completes and they haven't started the next action, the timer displays elapsed time in an "overtime" format to help them realize they should start their break or next work session.

**Why this priority**: This is the core value of the feature - providing immediate visual feedback that a session has ended and time is passing without action.

**Independent Test**: Can be fully tested by completing a 25-minute work session, waiting for 30 seconds without clicking any buttons, and verifying the display shows "-00:30" in red.

**Acceptance Scenarios**:

1. **Given** work session has completed, **When** 1 minute and 15 seconds have passed without user action, **Then** timer displays "-01:15" in red color
2. **Given** work session has completed, **When** 10 seconds have passed, **Then** timer displays "-00:10" in red color
3. **Given** work session has completed and overtime is showing "-00:45", **When** user clicks Start/Pause, **Then** overtime display clears and timer starts normally

---

### User Story 2 - Break Session Overtime Display (Priority: P2)

When a user's break session completes and they haven't started their next work session, the timer displays elapsed time in overtime format to remind them to return to work.

**Why this priority**: Provides consistency with work session behavior and helps users manage their break time effectively.

**Independent Test**: Can be fully tested by completing a 5-minute break session, waiting for 1 minute without action, and verifying the display shows "-01:00" in red.

**Acceptance Scenarios**:

1. **Given** break session has completed, **When** 2 minutes and 30 seconds have passed without user action, **Then** timer displays "-02:30" in red color
2. **Given** break session has completed and overtime is showing "-01:20", **When** user clicks Start/Pause, **Then** overtime display clears and new work session starts

---

### Edge Cases

- What happens when overtime exceeds 59 minutes and 59 seconds?
  - No, please stop at "-59:59", limit value.
- What happens if user clicks Clear while overtime is displayed? Timer resets to initial state (25:00 for work mode)
  - Yes, restore initial state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST display elapsed time in MM:SS format after any session (work or break) completes
- **FR-002**: System MUST prefix elapsed time display with a minus sign (e.g., "-02:13")
- **FR-003**: System MUST display elapsed time in red color to differentiate from normal timer display
- **FR-004**: System MUST update elapsed time display every second to show accurate overtime
- **FR-005**: System MUST reset elapsed time display to normal timer when user starts a new session
- **FR-006**: System MUST cap overtime display at 59:59 (display freezes at "-59:59" for any elapsed time beyond 3599 seconds)
- **FR-007**: System MUST maintain elapsed time accuracy even if app is minimized or in background

### Key Entities

- **Timer State**: Extended to include "overtime" state when session completes without user action
- **Display Format**: Time representation with sign prefix (negative for overtime) and color coding (red for overtime, default for normal). Always uses MM:SS format, capped at -59:59.
- **Session Completion**: Timestamp marking when a work or break session reached zero, used as reference for calculating elapsed overtime

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can immediately recognize when a session has ended by observing the red negative time display
- **SC-002**: Elapsed time updates are visible within 1 second of the actual time passage
- **SC-003**: 100% of overtime scenarios (work and break) display elapsed time correctly in red with minus prefix
- **SC-004**: Timer correctly caps overtime display at -59:59 and maintains this value for any elapsed time beyond 3599 seconds

## Assumptions & Constraints

### Assumptions

- Red color is sufficient visual indicator for overtime state (no additional sounds or notifications required)
- Users understand negative time with minus prefix as "time passed since completion"
- Display format MM:SS with cap at -59:59 is sufficient for overtime tracking (extended periods beyond 1 hour will remain at -59:59)
- Timer continues running in background when app is minimized or unfocused
- System clock provides accurate time source for elapsed time calculations

### Constraints

- Must maintain consistency with existing Pomodoro Technique timings (25-minute work, 5-minute break)
- Visual changes limited to timer display area (no new UI elements required)
- Must work within existing desktop app window size constraints
- Display must remain readable in red color with current font and size
