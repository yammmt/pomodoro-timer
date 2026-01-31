import { invoke } from '@tauri-apps/api/core';

interface TimerState {
  phase: 'work' | 'break';
  status: 'workReady' | 'breakReady' | 'running' | 'paused' | 'complete';
  remainingSecs: number;
  durationSecs: number;
  completionFlag: boolean;
  stateLabel: string;
  overtimeSecs?: number;
}

const CHIME_DURATION_SEC = 3.0;

let pollInterval: number | null = null;
let lastCompletionFlag = false;
let audioContext: AudioContext | null = null;

let startBtn: HTMLButtonElement;
let pauseBtn: HTMLButtonElement;
let resumeBtn: HTMLButtonElement;
let clearBtn: HTMLButtonElement;
let workBtn: HTMLButtonElement;
let breakBtn: HTMLButtonElement;
let timerDisplay: HTMLDivElement;
let stateLabel: HTMLDivElement;
let confirmDialog: HTMLDivElement;
let confirmClearBtn: HTMLButtonElement;
let cancelClearBtn: HTMLButtonElement;

function shouldConfirmClear(state: TimerState): boolean {
  // Confirm if timer is running or paused, OR if in Ready state with paused time
  const hasActivity = state.status === 'running' || state.status === 'paused';
  const hasPausedTime = (state.status === 'workReady' || state.status === 'breakReady')
    && state.remainingSecs !== state.durationSecs;
  return hasActivity || hasPausedTime;
}

function showClearConfirmDialog() {
  confirmDialog.style.display = 'flex';
  confirmClearBtn.focus();
}

function hideClearConfirmDialog() {
  confirmDialog.style.display = 'none';
}

function playCompletionChime() {
  try {
    // Create AudioContext on first use (required for user gesture in some browsers)
    if (!audioContext) {
      audioContext = new AudioContext();
    }

    // Resume context if suspended (browser autoplay policy)
    if (audioContext.state === 'suspended') {
      audioContext.resume();
    }

    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(audioContext.destination);

    // Pleasant chime: 880Hz (A5)
    oscillator.frequency.value = 880;
    oscillator.type = 'sine';

    // Fade out to avoid click
    gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
    gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + CHIME_DURATION_SEC);

    oscillator.start(audioContext.currentTime);
    oscillator.stop(audioContext.currentTime + CHIME_DURATION_SEC);
  } catch (err) {
    console.warn('Could not play chime:', err);
  }
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

async function updateUI() {
  try {
    const state = await invoke<TimerState>('get_state');

    // Handle overtime display
    if (state.overtimeSecs !== undefined) {
      timerDisplay.textContent = `-${formatTime(state.overtimeSecs)}`;
      timerDisplay.classList.add('overtime');
    } else {
      timerDisplay.textContent = formatTime(state.remainingSecs);
      timerDisplay.classList.remove('overtime');
    }

    stateLabel.textContent = state.stateLabel;

    // Sync active mode button with backend phase
    if (state.phase === 'work') {
      workBtn.classList.add('active');
      breakBtn.classList.remove('active');
    } else {
      breakBtn.classList.add('active');
      workBtn.classList.remove('active');
    }

    // Update button states - Start enabled when in Ready states
    startBtn.disabled = !(state.status === 'workReady' || state.status === 'breakReady');
    pauseBtn.disabled = state.status !== 'running';
    resumeBtn.disabled = state.status !== 'paused';
    // Clear enabled unless in fresh Ready state (remaining time = full duration)
    const isFreshState = ((state.status === 'workReady' || state.status === 'breakReady')
      && state.remainingSecs === state.durationSecs);
    clearBtn.disabled = isFreshState;

    // Detect completion transitions and play chime
    if (state.completionFlag && !lastCompletionFlag) {
      playCompletionChime();
    }

    lastCompletionFlag = state.completionFlag;
  } catch (error) {
    console.error('Failed to get state:', error);
  }
}

function attachEventListeners() {
  startBtn.addEventListener('click', async () => {
    try {
      await invoke('start_timer');
      await updateUI();
      startPolling();
    } catch (error) {
      console.error('Failed to start timer:', error);
    }
  });

  pauseBtn.addEventListener('click', async () => {
    try {
      await invoke('pause_timer');
      await updateUI();
    } catch (error) {
      console.error('Failed to pause timer:', error);
    }
  });

  resumeBtn.addEventListener('click', async () => {
    try {
      await invoke('resume_timer');
      await updateUI();
    } catch (error) {
      console.error('Failed to resume timer:', error);
    }
  });

  clearBtn.addEventListener('click', async () => {
    try {
      const state = await invoke<TimerState>('get_state');

      if (shouldConfirmClear(state)) {
        showClearConfirmDialog();
      } else {
        // Directly clear without confirmation for Ready states
        await invoke('clear_timer');
        await updateUI();
        stopPolling();
      }
    } catch (error) {
      console.error('Failed to get state:', error);
    }
  });

  confirmClearBtn.addEventListener('click', async () => {
    try {
      await invoke('clear_timer');
      hideClearConfirmDialog();
      await updateUI();
      stopPolling();
    } catch (error) {
      console.error('Failed to clear timer:', error);
    }
  });

  cancelClearBtn.addEventListener('click', () => {
    hideClearConfirmDialog();
  });

  confirmDialog.addEventListener('click', (event) => {
    if (event.target === confirmDialog) {
      hideClearConfirmDialog();
    }
  });

  workBtn.addEventListener('click', async () => {
    try {
      await invoke('set_phase', { phase: 'work' });
      await updateUI();
    } catch (error) {
      console.error('Failed to set work phase:', error);
    }
  });

  breakBtn.addEventListener('click', async () => {
    try {
      await invoke('set_phase', { phase: 'break' });
      await updateUI();
    } catch (error) {
      console.error('Failed to set break phase:', error);
    }
  });

  document.addEventListener('keydown', (event) => {
    if (event.key === 'Escape' && confirmDialog.style.display === 'flex') {
      hideClearConfirmDialog();
    }
  });
}

function startPolling() {
  if (pollInterval === null) {
    pollInterval = window.setInterval(updateUI, 1000); // 1 Hz polling
  }
}

function stopPolling() {
  if (pollInterval !== null) {
    clearInterval(pollInterval);
    pollInterval = null;
  }
}

// Wait for DOM to be ready before initializing
document.addEventListener('DOMContentLoaded', () => {
  // Get DOM elements
  startBtn = document.getElementById('start-btn') as HTMLButtonElement;
  pauseBtn = document.getElementById('pause-btn') as HTMLButtonElement;
  resumeBtn = document.getElementById('resume-btn') as HTMLButtonElement;
  clearBtn = document.getElementById('clear-btn') as HTMLButtonElement;
  workBtn = document.getElementById('work-btn') as HTMLButtonElement;
  breakBtn = document.getElementById('break-btn') as HTMLButtonElement;
  timerDisplay = document.getElementById('timer-display') as HTMLDivElement;
  stateLabel = document.getElementById('state-label') as HTMLDivElement;
  confirmDialog = document.getElementById('clear-confirm-dialog') as HTMLDivElement;
  confirmClearBtn = document.getElementById('confirm-clear-btn') as HTMLButtonElement;
  cancelClearBtn = document.getElementById('cancel-clear-btn') as HTMLButtonElement;

  // Attach event listeners
  attachEventListeners();

  // Initialize UI
  updateUI();

  console.log('Pomodoro Timer initialized');
});
