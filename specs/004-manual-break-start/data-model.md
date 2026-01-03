# Data Model: Manual Break Start

**Feature**: Manual Break Start  
**Date**: 2026-01-03  
**Purpose**: Define data structures and state machine for timer behavior

## Entities

### TimerState

**Purpose**: Represents the current state of the Pomodoro timer, including phase, status, remaining time, and metadata.

**Attributes**:
- `phase`: Enum - Current timer phase (Work or Break)
- `status`: Enum - Current execution status (WorkReady, BreakReady, Running, Paused, Complete)
- `remaining_secs`: u32 - Seconds remaining in current countdown (0 when in ready states)
- `duration_secs`: u32 - Total duration for current phase (1500 for work, 300 for break)
- `completion_flag`: bool - True when a session just completed (triggers chime)
- `state_label`: String - Human-readable state description for UI display

**Serialization**: JSON via serde, camelCase for frontend compatibility

**State Transitions**: See State Machine diagram below

### Phase (Enum)

**Values**:
- `Work`: 25-minute focus session (1500 seconds, currently 5 for testing)
- `Break`: 5-minute rest session (300 seconds, currently 3 for testing)

**Serialization**: lowercase string ("work", "break")

**Immutability**: Phase value determines duration but doesn't change during a countdown

### Status (Enum)

**Values**:
- `WorkReady`: Timer ready to start a work session (initial state, or after break completes)
- `BreakReady`: Timer ready to start a break session (after work completes)
- `Running`: Countdown actively decrementing
- `Paused`: Countdown temporarily stopped, preserving remaining time
- `Complete`: (Legacy) Session fully finished - may be deprecated in favor of Ready states

**Serialization**: camelCase string ("workReady", "breakReady", "running", "paused", "complete")

**Transitions**: Controlled by user commands (start, pause, resume, clear) and completion events

## State Machine

```
┌─────────────────────────────────────────────────────────────────┐
│                         WORK CYCLE                              │
└─────────────────────────────────────────────────────────────────┘

    [Initial/Clear]
          │
          ▼
    ┌──────────┐
    │WorkReady │  status=WorkReady, phase=Work, remaining=1500
    └──────────┘
          │
       [start]
          │
          ▼
    ┌──────────┐
    │ Running  │  status=Running, phase=Work, remaining=1500→0
    │  (Work)  │
    └──────────┘
          │ ◄───[resume]───┐
       [pause]             │
          │                │
          ▼                │
    ┌──────────┐           │
    │  Paused  │           │
    │  (Work)  │           │
    └──────────┘───────────┘
          │
    [remaining=0]
          │
          ▼
    ┌──────────┐
    │BreakReady│  status=BreakReady, phase=Break, remaining=300
    │          │  completion_flag=true, plays chime
    └──────────┘
          │
       [start]
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                        BREAK CYCLE                              │
└─────────────────────────────────────────────────────────────────┘

    ┌──────────┐
    │ Running  │  status=Running, phase=Break, remaining=300→0
    │ (Break)  │
    └──────────┘
          │ ◄───[resume]───┐
       [pause]             │
          │                │
          ▼                │
    ┌──────────┐           │
    │  Paused  │           │
    │ (Break)  │           │
    └──────────┘───────────┘
          │
    [remaining=0]
          │
          ▼
    ┌──────────┐
    │WorkReady │  status=WorkReady, phase=Work, remaining=1500
    │          │  completion_flag=true, plays chime
    └──────────┘
          │
          └──── [cycle repeats]

┌─────────────────────────────────────────────────────────────────┐
│                     CLEAR TRANSITIONS                           │
└─────────────────────────────────────────────────────────────────┘

    Any State ──[clear]──> WorkReady
                            (resets to initial work state)
```

## Validation Rules

### State Invariants

1. **Ready States**: When status is `WorkReady` or `BreakReady`:
   - `remaining_secs` equals `duration_secs` (full duration shown)
   - No active countdown (no `started_instant` in backend)
   - User must press Start to transition to Running

2. **Running State**: When status is `Running`:
   - `started_instant` exists (backend tracking)
   - `remaining_secs` decrements each second
   - When `remaining_secs` reaches 0, auto-transition to Ready state

3. **Paused State**: When status is `Paused`:
   - `remaining_secs` preserved from pause moment
   - `paused_remaining` saved in backend
   - Resume restores countdown from saved time

4. **Completion Flag**: Set to `true` when:
   - Work countdown completes → transition to BreakReady
   - Break countdown completes → transition to WorkReady
   - Cleared on next Start or Clear command

### Phase-Status Combinations (Valid)

| Phase | Status | Valid? | Meaning |
|-------|--------|--------|---------|
| Work | WorkReady | ✅ | Ready to start work session |
| Work | Running | ✅ | Work countdown active |
| Work | Paused | ✅ | Work countdown paused |
| Break | BreakReady | ✅ | Ready to start break session |
| Break | Running | ✅ | Break countdown active |
| Break | Paused | ✅ | Break countdown paused |
| Work | BreakReady | ❌ | Invalid - phase mismatch |
| Break | WorkReady | ❌ | Invalid - phase mismatch |

**Note**: Phase and Status must be consistent. Ready state determines phase for next countdown.

## Relationships

### TimerState → Phase
- **Cardinality**: 1:1 (one state has one phase at a time)
- **Nature**: Composition (phase is part of state)

### TimerState → Status
- **Cardinality**: 1:1 (one state has one status at a time)
- **Nature**: Composition (status is part of state)

### Frontend ↔ Backend
- **Protocol**: Tauri IPC commands (JSON-RPC style)
- **Frequency**: Frontend polls every 1 second for state updates
- **Direction**: Frontend sends commands, backend returns TimerState

## Data Flow

```
Frontend (main.ts)
    │
    │ invoke('start_timer')
    ▼
Backend Command (lib.rs)
    │
    │ timer.lock().start()
    ▼
TimerService (timer.rs)
    │
    │ Update state, return TimerState
    ▼
JSON Serialization (serde)
    │
    │ {"phase":"work","status":"running",...}
    ▼
Frontend Polling (updateUI)
    │
    │ Render time, update buttons
    │ Check completion_flag → play chime
    ▼
UI Display
```

## Backend-Only State

**Not serialized to frontend**:
- `started_instant`: Option<Instant> - Internal time tracking for countdown calculations
- `paused_remaining`: Option<u32> - Cached remaining time when paused

**Reason**: Frontend only needs computed `remaining_secs`. Internal timing mechanism is implementation detail.

## Examples

### Example 1: Fresh Start
```json
{
  "phase": "work",
  "status": "workReady",
  "remainingSecs": 1500,
  "durationSecs": 1500,
  "completionFlag": false,
  "stateLabel": "Ready to work"
}
```

### Example 2: Work Running (10 seconds elapsed)
```json
{
  "phase": "work",
  "status": "running",
  "remainingSecs": 1490,
  "durationSecs": 1500,
  "completionFlag": false,
  "stateLabel": "Working"
}
```

### Example 3: Work Completed (Ready for Break)
```json
{
  "phase": "break",
  "status": "breakReady",
  "remainingSecs": 300,
  "durationSecs": 300,
  "completionFlag": true,
  "stateLabel": "Break ready - press Start"
}
```

### Example 4: Break Running
```json
{
  "phase": "break",
  "status": "running",
  "remainingSecs": 285,
  "durationSecs": 300,
  "completionFlag": false,
  "stateLabel": "Break time"
}
```

### Example 5: Break Completed (Ready for Work)
```json
{
  "phase": "work",
  "status": "workReady",
  "remainingSecs": 1500,
  "durationSecs": 1500,
  "completionFlag": true,
  "stateLabel": "Work ready - press Start"
}
```

## Migration Notes

**Existing Status Values**:
- `Idle` → renamed to `WorkReady` (semantic clarity)
- `Complete` → may be replaced by Ready states

**Backwards Compatibility**: None required - internal desktop app, no persisted data or external API consumers.

**Database Changes**: N/A - no persistent storage
