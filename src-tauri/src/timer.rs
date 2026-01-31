use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Instant;

const WORK_DURATION_SECS: u32 = 1500; // 25 minutes
const BREAK_DURATION_SECS: u32 = 300; // 5 minutes

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Work,
    Break,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    WorkReady,
    BreakReady,
    Running,
    Paused,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerState {
    pub phase: Phase,
    pub status: Status,
    pub remaining_secs: u32,
    pub duration_secs: u32,
    pub completion_flag: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_at: Option<String>,
    pub state_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_secs: Option<u32>,
}

pub struct TimerService {
    pub(crate) phase: Phase,
    pub(crate) status: Status,
    pub(crate) remaining_secs: u32,
    duration_secs: u32,
    completion_flag: bool,
    pub(crate) started_instant: Option<Instant>,
    pub(crate) completed_at: Option<Instant>,
    pub(crate) paused_work_secs: Option<u32>,
    pub(crate) paused_break_secs: Option<u32>,
    state_label: String,
}

impl TimerService {
    pub fn new() -> Self {
        Self {
            phase: Phase::Work,
            status: Status::WorkReady,
            remaining_secs: WORK_DURATION_SECS,
            duration_secs: WORK_DURATION_SECS,
            completion_flag: false,
            started_instant: None,
            completed_at: None,
            paused_work_secs: None,
            paused_break_secs: None,
            state_label: "Ready to work".to_string(),
        }
    }

    pub fn get_state(&mut self) -> TimerState {
        self.update_remaining();
        TimerState {
            phase: self.phase,
            status: self.status,
            remaining_secs: self.remaining_secs,
            duration_secs: self.duration_secs,
            completion_flag: self.completion_flag,
            started_at: None,
            paused_at: None,
            state_label: self.state_label.clone(),
            overtime_secs: None,
        }
    }

    pub(crate) fn update_remaining(&mut self) {
        if self.status == Status::Running {
            if let Some(start) = self.started_instant {
                let elapsed = start.elapsed().as_secs() as u32;
                let initial = match self.phase {
                    Phase::Work => self.paused_work_secs.unwrap_or(self.duration_secs),
                    Phase::Break => self.paused_break_secs.unwrap_or(self.duration_secs),
                };

                if elapsed >= initial {
                    self.remaining_secs = 0;
                    self.handle_completion();
                } else {
                    self.remaining_secs = initial - elapsed;
                }
            }
        }
    }

    pub(crate) fn handle_completion(&mut self) {
        self.completion_flag = true;
        self.remaining_secs = 0;
        self.status = Status::Complete;
        self.started_instant = None;

        // Stay in current phase, update label
        self.state_label = match self.phase {
            Phase::Work => "Work completed".to_string(),
            Phase::Break => "Break completed".to_string(),
        };

        // Note: Do NOT clear paused_work_secs or paused_break_secs
        // Note: Do NOT change self.phase or self.duration_secs
    }

    pub fn start(&mut self) -> Result<TimerState, String> {
        if self.status == Status::Running {
            return Err("Timer already running".to_string());
        }

        // Phase-aware start: start work or break based on current status
        match self.status {
            Status::WorkReady => {
                // Start work session
                self.phase = Phase::Work;
                self.status = Status::Running;
                self.duration_secs = WORK_DURATION_SECS;
                self.remaining_secs = WORK_DURATION_SECS;
                self.completion_flag = false;
                self.state_label = "Working".to_string();
                self.started_instant = Some(Instant::now());
                self.paused_work_secs = None;
                // Preserve paused_break_secs for switching back to break later
            }
            Status::Complete => {
                // Restart the current phase (stay in work or break)
                match self.phase {
                    Phase::Work => {
                        self.status = Status::Running;
                        self.duration_secs = WORK_DURATION_SECS;
                        self.remaining_secs = WORK_DURATION_SECS;
                        self.completion_flag = false;
                        self.state_label = "Working".to_string();
                        self.started_instant = Some(Instant::now());
                        self.paused_work_secs = None;
                    }
                    Phase::Break => {
                        self.status = Status::Running;
                        self.duration_secs = BREAK_DURATION_SECS;
                        self.remaining_secs = BREAK_DURATION_SECS;
                        self.completion_flag = false;
                        self.state_label = "Break time".to_string();
                        self.started_instant = Some(Instant::now());
                        self.paused_break_secs = None;
                    }
                }
            }
            Status::BreakReady => {
                // Start break session
                self.phase = Phase::Break;
                self.status = Status::Running;
                self.duration_secs = BREAK_DURATION_SECS;
                self.remaining_secs = BREAK_DURATION_SECS;
                self.completion_flag = false;
                self.state_label = "Break time".to_string();
                self.started_instant = Some(Instant::now());
                self.paused_break_secs = None;
                // Preserve paused_work_secs for switching back to work later
            }
            Status::Running => {
                return Err("Timer already running".to_string());
            }
            Status::Paused => {
                return Err("Timer is paused, use resume instead".to_string());
            }
        }

        Ok(self.get_state())
    }

    pub fn pause(&mut self) -> Result<TimerState, String> {
        if self.status != Status::Running {
            return Err("No running timer to pause".to_string());
        }

        self.update_remaining();
        self.status = Status::Paused;
        match self.phase {
            Phase::Work => self.paused_work_secs = Some(self.remaining_secs),
            Phase::Break => self.paused_break_secs = Some(self.remaining_secs),
        }
        self.started_instant = None;
        self.state_label = format!(
            "Paused ({})",
            if self.phase == Phase::Work {
                "work"
            } else {
                "break"
            }
        );

        Ok(self.get_state())
    }

    pub fn resume(&mut self) -> Result<TimerState, String> {
        if self.status != Status::Paused {
            return Err("No paused timer to resume".to_string());
        }

        self.status = Status::Running;
        self.started_instant = Some(Instant::now());
        self.state_label = if self.phase == Phase::Work {
            "Working".to_string()
        } else {
            "Break time".to_string()
        };

        Ok(self.get_state())
    }

    pub fn clear(&mut self) -> Result<TimerState, String> {
        // Preserve current phase, reset to ready state
        match self.phase {
            Phase::Work => {
                self.status = Status::WorkReady;
                self.remaining_secs = WORK_DURATION_SECS;
                self.duration_secs = WORK_DURATION_SECS;
                self.state_label = "Ready to work".to_string();
            }
            Phase::Break => {
                self.status = Status::BreakReady;
                self.remaining_secs = BREAK_DURATION_SECS;
                self.duration_secs = BREAK_DURATION_SECS;
                self.state_label = "Ready to break".to_string();
            }
        }

        self.completion_flag = false;
        self.started_instant = None;
        self.paused_work_secs = None;
        self.paused_break_secs = None;

        Ok(self.get_state())
    }

    pub fn set_phase(&mut self, new_phase: Phase) {
        // Idempotent: no-op if already on requested phase
        if new_phase == self.phase {
            return;
        }

        // If currently running, pause and save remaining time
        if self.status == Status::Running {
            self.status = Status::Paused;
            // Store current remaining in exiting phase's field
            match self.phase {
                Phase::Work => self.paused_work_secs = Some(self.remaining_secs),
                Phase::Break => self.paused_break_secs = Some(self.remaining_secs),
            }
            self.started_instant = None;
        } else if self.status == Status::Paused {
            // Already paused; save current remaining time to exiting phase
            match self.phase {
                Phase::Work => self.paused_work_secs = Some(self.remaining_secs),
                Phase::Break => self.paused_break_secs = Some(self.remaining_secs),
            }
        } else if self.status == Status::Complete {
            // Session completed; switching phase clears completion state
            // No need to save remaining time (already 0)
            // Completion flag will be cleared below
        }

        // Switch to new phase
        self.phase = new_phase;

        // Load paused time from new phase, or use standard duration
        match new_phase {
            Phase::Work => {
                self.duration_secs = WORK_DURATION_SECS;
                self.remaining_secs = self.paused_work_secs.unwrap_or(WORK_DURATION_SECS);
                // Set status to Paused if we have paused time, otherwise WorkReady
                if self.paused_work_secs.is_some() {
                    self.status = Status::Paused;
                    self.state_label = "Paused (work)".to_string();
                } else {
                    self.status = Status::WorkReady;
                    self.state_label = "Ready to work".to_string();
                }
            }
            Phase::Break => {
                self.duration_secs = BREAK_DURATION_SECS;
                self.remaining_secs = self.paused_break_secs.unwrap_or(BREAK_DURATION_SECS);
                // Set status to Paused if we have paused time, otherwise BreakReady
                if self.paused_break_secs.is_some() {
                    self.status = Status::Paused;
                    self.state_label = "Paused (break)".to_string();
                } else {
                    self.status = Status::BreakReady;
                    self.state_label = "Ready to break".to_string();
                }
            }
        }

        self.completion_flag = false;
    }
}

pub type SharedTimerService = Mutex<TimerService>;

pub fn create_timer_service() -> SharedTimerService {
    Mutex::new(TimerService::new())
}

#[cfg(test)]
mod tests;
