import { invoke } from '@tauri-apps/api/core';

interface TimerState {
  phase: 'work' | 'break';
  status: 'idle' | 'running' | 'paused' | 'complete';
  remainingSecs: number;
  durationSecs: number;
  completionFlag: boolean;
  stateLabel: string;
}

const CHIME_DURATION_SEC = 3.0;

let pollInterval: number | null = null;
let lastCompletionFlag = false;
let audioContext: AudioContext | null = null;

let startBtn: HTMLButtonElement;
let pauseBtn: HTMLButtonElement;
let resumeBtn: HTMLButtonElement;
let clearBtn: HTMLButtonElement;
let timerDisplay: HTMLDivElement;
let stateLabel: HTMLDivElement;

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

    timerDisplay.textContent = formatTime(state.remainingSecs);
    stateLabel.textContent = state.stateLabel;

    // Update button states
    startBtn.disabled = state.status === 'running';
    pauseBtn.disabled = state.status !== 'running';
    resumeBtn.disabled = state.status !== 'paused';
    clearBtn.disabled = false;

    // Detect completion transitions and play chime
    if (state.completionFlag && !lastCompletionFlag) {
      playCompletionChime();
    }
    // Also detect work->break transition (completion of work phase)
    if (state.phase === 'break' && state.status === 'running' && state.remainingSecs === state.durationSecs) {
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
      await invoke('clear_timer');
      await updateUI();
      stopPolling();
    } catch (error) {
      console.error('Failed to clear timer:', error);
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
  timerDisplay = document.getElementById('timer-display') as HTMLDivElement;
  stateLabel = document.getElementById('state-label') as HTMLDivElement;

  // Attach event listeners
  attachEventListeners();

  // Initialize UI
  updateUI();

  console.log('Pomodoro Timer initialized');
});
