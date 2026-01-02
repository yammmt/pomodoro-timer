# Data Model — Pomodoro Desktop Timer

## Entities

### TimerSession

- type: `work` | `break`
- duration_secs: integer (work 1500, break 300)
- remaining_secs: integer >= 0
- state: `idle` | `running` | `paused` | `complete`
- started_at: monotonic timestamp (set when entering running)
- paused_at: monotonic timestamp (set when entering paused)

### CycleState

- phase: `work` | `break`
- status: `idle` | `running` | `paused` | `complete`
- last_transition_at: monotonic timestamp
- completion_flag: boolean (true after break completion until cleared)

## Rules & Validation

- duration_secs fixed: work = 1500s, break = 300s.
- Start allowed only when status is `idle` or `complete` (after break) to begin a new work session.
- Pause allowed only when status is `running`.
- Resume allowed only when status is `paused`.
- Clear allowed in any state and resets to `phase=work`, `status=idle`, `remaining_secs=1500`, `completion_flag=false`.
- Automatic transition: when work `remaining_secs` reaches 0, transition to `phase=break`, `status=running`, `remaining_secs=300`.
- Completion: when break `remaining_secs` reaches 0, set `status=complete`, `completion_flag=true`, and keep `remaining_secs=0` until clear/start.

## State Transitions

- idle(work) → start → running(work)
- running(work) → pause → paused(work)
- paused(work) → resume → running(work)
- running(work) → elapse_to_zero → running(break)
- running(break) → pause → paused(break)
- paused(break) → resume → running(break)
- running(break) → elapse_to_zero → complete(break)
- complete(any) → start → running(work) with full work duration
- any → clear → idle(work)

## Derived UI Values

- display_time: formatted `MM:SS` from remaining_secs
- is_start_enabled: true when status in `idle` or `complete`
- is_pause_enabled: true when status is `running`
- is_resume_enabled: true when status is `paused`
- state_label: `Work`, `Break`, or `Ready`
