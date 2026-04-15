<!-- Caminho relativo: src/display/AnswerOption.svelte -->
<!-- Uma alternativa de resposta exibida no projetor -->
<script lang="ts">
  interface Props {
    letra: string;
    texto: string;
    state: "normal" | "selected" | "correct" | "incorrect" | "eliminated";
  }

  let { letra, texto, state }: Props = $props();

  let colorVar = $derived(
    `var(--color-option-${letra.toLowerCase()})`
  );
</script>

<div class="answer-option {state}" style="--option-color: {colorVar}">
  <div class="letter-badge">
    {letra}
  </div>
  <div class="answer-text">
    {texto}
  </div>
  {#if state === "correct"}
    <div class="result-icon correct-icon">✓</div>
  {:else if state === "incorrect"}
    <div class="result-icon incorrect-icon">✕</div>
  {/if}
</div>

<style>
  .answer-option {
    display: flex;
    align-items: center;
    gap: var(--spacing-md);
    padding: var(--spacing-md) var(--spacing-lg);
    background: var(--color-bg-card);
    border-radius: var(--border-radius-lg);
    border: 3px solid transparent;
    transition: all var(--transition-normal);
    position: relative;
    overflow: hidden;
    min-height: 70px;
  }

  .answer-option::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 5px;
    background: var(--option-color);
  }

  .letter-badge {
    width: 45px;
    height: 45px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--option-color);
    color: white;
    font-size: 1.25rem;
    font-weight: 900;
    border-radius: var(--border-radius-full);
    flex-shrink: 0;
  }

  .answer-text {
    flex: 1;
    font-size: 1.2rem;
    font-weight: 700;
    line-height: 1.3;
  }

  .result-icon {
    font-size: 2rem;
    font-weight: 900;
    flex-shrink: 0;
  }

  .correct-icon {
    color: var(--color-success);
  }

  .incorrect-icon {
    color: var(--color-error);
  }

  /* States */
  .selected {
    border-color: var(--color-accent-blue);
    background: rgba(33, 150, 243, 0.15);
    transform: scale(1.02);
    box-shadow: 0 0 20px rgba(33, 150, 243, 0.3);
  }

  .correct {
    border-color: var(--color-success);
    background: rgba(76, 175, 80, 0.2);
    transform: scale(1.03);
    box-shadow: 0 0 25px rgba(76, 175, 80, 0.4);
    animation: correctPulse 0.6s ease;
  }

  .incorrect {
    border-color: var(--color-error);
    background: rgba(244, 67, 54, 0.2);
    box-shadow: 0 0 20px rgba(244, 67, 54, 0.3);
    animation: shake 0.5s ease;
  }

  .eliminated {
    opacity: 0.2;
    transform: scale(0.95);
    filter: grayscale(1);
  }

  @keyframes correctPulse {
    0% { transform: scale(1); }
    50% { transform: scale(1.05); }
    100% { transform: scale(1.03); }
  }

  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    20% { transform: translateX(-8px); }
    40% { transform: translateX(8px); }
    60% { transform: translateX(-5px); }
    80% { transform: translateX(5px); }
  }
</style>
