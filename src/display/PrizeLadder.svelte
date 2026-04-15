<!-- Caminho relativo: src/display/PrizeLadder.svelte -->
<!-- Escada de prêmios visual no estilo Show do Milhão -->
<script lang="ts">
  import type { GameState } from "../lib/types";

  interface Props {
    gameState: GameState;
  }

  let { gameState }: Props = $props();

  let valores = $derived([...gameState.config.premio.valores].reverse());
  let totalPerguntas = $derived(gameState.config.premio.valores.length);

  function getStepClass(reverseIndex: number): string {
    const realIndex = totalPerguntas - 1 - reverseIndex;
    const isCurrent = realIndex === gameState.pergunta_atual;
    const isPassed = realIndex < gameState.pergunta_atual;
    const isCheckpoint = gameState.config.checkpoints.includes(realIndex);
    const isLast = realIndex === totalPerguntas - 1;

    let classes: string[] = [];
    if (isCurrent) classes.push("current");
    if (isPassed) classes.push("passed");
    if (isCheckpoint) classes.push("checkpoint");
    if (isLast) classes.push("final");

    return classes.join(" ");
  }

  function formatValue(value: number): string {
    return value.toLocaleString("pt-BR");
  }
</script>

<div class="prize-ladder">
  <div class="ladder-title">
    <span class="ladder-icon">{gameState.config.premio.icone}</span>
    <span>Prêmios</span>
  </div>

  <div class="ladder-steps">
    {#each valores as valor, i}
      <div class="ladder-step {getStepClass(i)}">
        <span class="step-number">{totalPerguntas - i}</span>
        <span class="step-value">
          {formatValue(valor)}
        </span>
        {#if gameState.config.checkpoints.includes(totalPerguntas - 1 - i)}
          <span class="checkpoint-marker">🔒</span>
        {/if}
      </div>
    {/each}
  </div>

  <div class="ladder-footer">
    <div class="prize-info">
      <span class="prize-label">Acumulado</span>
      <span class="prize-value">
        {gameState.config.premio.icone}
        {formatValue(gameState.premio_acumulado)}
      </span>
    </div>
    <div class="prize-info guaranteed">
      <span class="prize-label">Garantido</span>
      <span class="prize-value">
        {formatValue(gameState.premio_garantido)}
      </span>
    </div>
  </div>
</div>

<style>
  .prize-ladder {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--color-bg-secondary);
    border-radius: var(--border-radius-lg);
    border: 2px solid rgba(244, 160, 32, 0.15);
    overflow: hidden;
  }

  .ladder-title {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md);
    font-weight: 800;
    font-size: 1.1rem;
    color: var(--color-accent-gold);
    text-align: center;
    justify-content: center;
    border-bottom: 2px solid rgba(244, 160, 32, 0.15);
  }

  .ladder-icon {
    font-size: 1.5rem;
  }

  .ladder-steps {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-xs);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .ladder-step {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-xs) var(--spacing-sm);
    border-radius: var(--border-radius-sm);
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--color-text-secondary);
    transition: all var(--transition-fast);
    min-height: 32px;
  }

  .step-number {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-bg-primary);
    border-radius: var(--border-radius-full);
    font-size: 0.7rem;
    font-weight: 800;
    flex-shrink: 0;
  }

  .step-value {
    flex: 1;
    text-align: right;
  }

  .checkpoint-marker {
    font-size: 0.7rem;
    flex-shrink: 0;
  }

  /* States */
  .current {
    background: rgba(244, 160, 32, 0.2);
    color: var(--color-accent-gold);
    font-weight: 800;
    font-size: 0.9rem;
    border: 2px solid var(--color-accent-gold);
    animation: currentGlow 2s ease-in-out infinite;
  }

  .current .step-number {
    background: var(--color-accent-gold);
    color: var(--color-bg-primary);
  }

  @keyframes currentGlow {
    0%, 100% { box-shadow: 0 0 5px rgba(244, 160, 32, 0.3); }
    50% { box-shadow: 0 0 15px rgba(244, 160, 32, 0.5); }
  }

  .passed {
    color: var(--color-text-muted);
    opacity: 0.5;
  }

  .passed .step-number {
    background: var(--color-success);
    color: white;
  }

  .checkpoint {
    border-left: 3px solid var(--color-accent-gold);
  }

  .final {
    color: var(--color-accent-gold);
    font-weight: 800;
  }

  /* Footer */
  .ladder-footer {
    border-top: 2px solid rgba(244, 160, 32, 0.15);
    padding: var(--spacing-sm) var(--spacing-md);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .prize-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .prize-label {
    font-size: 0.75rem;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .prize-value {
    font-size: 0.9rem;
    font-weight: 800;
    color: var(--color-accent-gold);
  }

  .guaranteed .prize-value {
    color: var(--color-success);
  }
</style>
