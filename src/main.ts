import { invoke } from '@tauri-apps/api/core';

interface TimerState {
  phase: 'work' | 'break';
  status: 'idle' | 'running' | 'paused' | 'complete';
  remainingSecs: number;
  durationSecs: number;
  completionFlag: boolean;
  stateLabel: string;
}

let pollInterval: number | null = null;
let lastCompletionFlag = false;
let audio: HTMLAudioElement | null = null;

let startBtn: HTMLButtonElement;
let pauseBtn: HTMLButtonElement;
let resumeBtn: HTMLButtonElement;
let clearBtn: HTMLButtonElement;
let timerDisplay: HTMLDivElement;
let stateLabel: HTMLDivElement;

function initAudio() {
  // Create audio element with embedded chime
  audio = document.createElement('audio');
  audio.preload = 'auto';
  const source = document.createElement('source');
  source.src = 'data:audio/wav;base64,UklGRnoGAABXQVZFZm10IBAAAAABAAEAQB8AAEAfAAABAAgAZGF0YQoGAACBhYqFgYJ+fH+Cg4aCgX1+fn9/gH9+fX1+f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af39+fX5/f4B/gH+Af3=';
  source.type = 'audio/wav';
  audio.appendChild(source);
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
}

function playCompletionChime() {
  if (audio) {
    audio.currentTime = 0;
    audio.play().catch(err => console.warn('Could not play chime:', err));
  }
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

  // Initialize audio and UI
  initAudio();
  updateUI();

  console.log('Pomodoro Timer initialized');
});
