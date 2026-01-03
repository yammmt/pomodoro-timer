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
}

pub struct TimerService {
    pub(crate) phase: Phase,
    pub(crate) status: Status,
    pub(crate) remaining_secs: u32,
    duration_secs: u32,
    completion_flag: bool,
    pub(crate) started_instant: Option<Instant>,
    paused_remaining: Option<u32>,
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
            paused_remaining: None,
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
        }
    }

    pub(crate) fn update_remaining(&mut self) {
        if self.status == Status::Running {
            if let Some(start) = self.started_instant {
                let elapsed = start.elapsed().as_secs() as u32;
                let initial = self.paused_remaining.unwrap_or(self.duration_secs);

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
        match self.phase {
            Phase::Work => {
                // Transition to break ready (don't auto-start)
                self.phase = Phase::Break;
                self.status = Status::BreakReady;
                self.duration_secs = BREAK_DURATION_SECS;
                self.remaining_secs = BREAK_DURATION_SECS;
                self.state_label = "Break ready - press Start".to_string();
                self.started_instant = None;
                self.paused_remaining = None;
            }
            Phase::Break => {
                // Transition to work ready (don't auto-start)
                self.phase = Phase::Work;
                self.status = Status::WorkReady;
                self.duration_secs = WORK_DURATION_SECS;
                self.remaining_secs = WORK_DURATION_SECS;
                self.state_label = "Work ready - press Start".to_string();
                self.started_instant = None;
                self.paused_remaining = None;
            }
        }
    }

    pub fn start(&mut self) -> Result<TimerState, String> {
        if self.status == Status::Running {
            return Err("Timer already running".to_string());
        }

        // Phase-aware start: start work or break based on current status
        match self.status {
            Status::WorkReady | Status::Complete => {
                // Start work session
                self.phase = Phase::Work;
                self.status = Status::Running;
                self.duration_secs = WORK_DURATION_SECS;
                self.remaining_secs = WORK_DURATION_SECS;
                self.completion_flag = false;
                self.state_label = "Working".to_string();
                self.started_instant = Some(Instant::now());
                self.paused_remaining = None;
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
                self.paused_remaining = None;
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
        self.paused_remaining = Some(self.remaining_secs);
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
        self.phase = Phase::Work;
        self.status = Status::WorkReady;
        self.remaining_secs = WORK_DURATION_SECS;
        self.duration_secs = WORK_DURATION_SECS;
        self.completion_flag = false;
        self.started_instant = None;
        self.paused_remaining = None;
        self.state_label = "Ready to work".to_string();

        Ok(self.get_state())
    }
}

pub type SharedTimerService = Mutex<TimerService>;

pub fn create_timer_service() -> SharedTimerService {
    Mutex::new(TimerService::new())
}

#[cfg(test)]
mod tests;
