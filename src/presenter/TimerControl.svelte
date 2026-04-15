<!-- Caminho relativo: src/presenter/TimerControl.svelte -->
<!-- Controle do timer para o apresentador -->
<script lang="ts">
  import type { GameState } from "../lib/types";
  import { iniciarTimer, pausarTimer, tickTimer } from "../lib/api";

  interface Props {
    gameState: GameState | null;
    onStateUpdate: (state: GameState) => void;
  }

  let { gameState, onStateUpdate }: Props = $props();

  let timerInterval: ReturnType<typeof setInterval> | null = $state(null);

  let canStartTimer = $derived(
    gameState?.status === "MostrandoPergunta" && !gameState?.timer_ativo
  );

  let isTimerRunning = $derived(gameState?.timer_ativo ?? false);

  let timerDisplay = $derived(
    gameState?.timer_segundos_restantes !== null &&
      gameState?.timer_segundos_restantes !== undefined
      ? formatTime(gameState.timer_segundos_restantes)
      : "--:--"
  );

  function formatTime(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  }

  async function handleStartTimer() {
    try {
      const state = await iniciarTimer();
      onStateUpdate(state);

      // Iniciar intervalo de tick
      if (timerInterval) clearInterval(timerInterval);
      timerInterval = setInterval(async () => {
        try {
          const newState = await tickTimer();
          onStateUpdate(newState);
          if (!newState.timer_ativo) {
            if (timerInterval) clearInterval(timerInterval);
            timerInterval = null;
          }
        } catch {
          if (timerInterval) clearInterval(timerInterval);
          timerInterval = null;
        }
      }, 1000);
    } catch (e: any) {
      console.error("Erro ao iniciar timer:", e);
    }
  }

  async function handlePauseTimer() {
    try {
      if (timerInterval) {
        clearInterval(timerInterval);
        timerInterval = null;
      }
      const state = await pausarTimer();
      onStateUpdate(state);
    } catch (e: any) {
      console.error("Erro ao pausar timer:", e);
    }
  }

  // Cleanup on destroy (Svelte 5 uses $effect for cleanup)
  $effect(() => {
    return () => {
      if (timerInterval) {
        clearInterval(timerInterval);
      }
    };
  });
</script>

<div class="timer-panel">
  <h3>⏱️ Timer</h3>

  <div class="timer-display" class:active={isTimerRunning}>
    <span class="timer-value">{timerDisplay}</span>
  </div>

  <div class="timer-buttons">
    {#if !isTimerRunning}
      <button
        class="btn btn-timer-start"
        onclick={handleStartTimer}
        disabled={!canStartTimer}
      >
        ▶️ Iniciar Timer
      </button>
    {:else}
      <button class="btn btn-timer-pause" onclick={handlePauseTimer}>
        ⏸️ Pausar Timer
      </button>
    {/if}
  </div>
</div>

<style>
  .timer-panel {
    background: var(--color-bg-secondary);
    border-radius: var(--border-radius-lg);
    padding: var(--spacing-md);
    display: flex;
    align-items: center;
    gap: var(--spacing-lg);
  }

  .timer-panel h3 {
    font-size: 1rem;
    color: var(--color-accent-gold);
    white-space: nowrap;
  }

  .timer-display {
    font-family: monospace;
    font-size: 2rem;
    font-weight: 900;
    background: var(--color-bg-primary);
    padding: var(--spacing-sm) var(--spacing-lg);
    border-radius: var(--border-radius-md);
    color: var(--color-text-secondary);
    min-width: 120px;
    text-align: center;
  }

  .timer-display.active {
    color: var(--color-accent-gold);
    animation: pulse 1s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.7;
    }
  }

  .timer-value {
    letter-spacing: 2px;
  }

  .timer-buttons {
    display: flex;
    gap: var(--spacing-sm);
  }

  .btn {
    padding: var(--spacing-sm) var(--spacing-md);
    border-radius: var(--border-radius-md);
    font-weight: 700;
    font-size: 0.9rem;
    transition: all var(--transition-fast);
  }

  .btn-timer-start {
    background: var(--color-success);
    color: white;
  }

  .btn-timer-start:hover:not(:disabled) {
    background: #388e3c;
  }

  .btn-timer-pause {
    background: var(--color-warning);
    color: white;
  }

  .btn-timer-pause:hover:not(:disabled) {
    background: #f57c00;
  }
</style>
