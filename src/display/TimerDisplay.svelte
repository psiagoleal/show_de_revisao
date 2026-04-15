<!-- Caminho relativo: src/display/TimerDisplay.svelte -->
<!-- Exibição visual do timer no display -->
<script lang="ts">
  interface Props {
    segundos: number;
    ativo: boolean;
  }

  let { segundos, ativo }: Props = $props();

  let formattedTime = $derived(() => {
    const mins = Math.floor(segundos / 60);
    const secs = segundos % 60;
    return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  });

  let isLow = $derived(segundos <= 10);
  let isCritical = $derived(segundos <= 5);
</script>

<div
  class="timer-display"
  class:active={ativo}
  class:low={isLow}
  class:critical={isCritical}
>
  <div class="timer-icon">⏱️</div>
  <div class="timer-value">{formattedTime()}</div>
</div>

<style>
  .timer-display {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
    padding: var(--spacing-md) var(--spacing-xl);
    background: var(--color-bg-secondary);
    border-radius: var(--border-radius-lg);
    border: 2px solid rgba(255, 255, 255, 0.1);
  }

  .timer-icon {
    font-size: 1.5rem;
  }

  .timer-value {
    font-family: monospace;
    font-size: 2.5rem;
    font-weight: 900;
    color: var(--color-text-secondary);
    letter-spacing: 3px;
  }

  .active .timer-value {
    color: var(--color-accent-gold);
  }

  .low .timer-value {
    color: var(--color-warning);
  }

  .critical .timer-value {
    color: var(--color-error);
    animation: criticalPulse 0.5s ease-in-out infinite;
  }

  @keyframes criticalPulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.6; transform: scale(1.05); }
  }
</style>
