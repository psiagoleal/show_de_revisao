<!-- Caminho relativo: src/presenter/PresenterControls.svelte -->
<!-- Controles principais do jogo para o apresentador -->
<script lang="ts">
  import type { GameState } from "../lib/types";
  import {
    selecionarResposta,
    revelarResposta,
    proximaPergunta,
    pararJogo,
  } from "../lib/api";

  interface Props {
    gameState: GameState | null;
    onStateUpdate: (state: GameState) => void;
  }

  let { gameState, onStateUpdate }: Props = $props();

  let canSelect = $derived(gameState?.status === "MostrandoPergunta");
  let canReveal = $derived(gameState?.status === "RespostaSelecionada");
  let canAdvance = $derived(
    gameState?.status === "RespostaRevelada" &&
      gameState?.resultado_final === null
  );
  let canStop = $derived(
    gameState?.status === "MostrandoPergunta" ||
      gameState?.status === "RespostaSelecionada"
  );

  async function handleSelect(opcao: string) {
    try {
      const state = await selecionarResposta(opcao);
      onStateUpdate(state);
    } catch (e: any) {
      console.error("Erro ao selecionar resposta:", e);
    }
  }

  async function handleReveal() {
    try {
      const state = await revelarResposta();
      onStateUpdate(state);
    } catch (e: any) {
      console.error("Erro ao revelar resposta:", e);
    }
  }

  async function handleNext() {
    try {
      const state = await proximaPergunta();
      onStateUpdate(state);
    } catch (e: any) {
      console.error("Erro ao avançar:", e);
    }
  }

  async function handleStop() {
    try {
      const state = await pararJogo();
      onStateUpdate(state);
    } catch (e: any) {
      console.error("Erro ao parar:", e);
    }
  }
</script>

<div class="controls-panel">
  <h3>🎮 Controles</h3>

  <div class="select-buttons">
    <span class="controls-label">Selecionar Resposta:</span>
    <div class="button-row">
      {#each ["A", "B", "C", "D"] as opcao}
        <button
          class="btn btn-option btn-option-{opcao.toLowerCase()}"
          onclick={() => handleSelect(opcao)}
          disabled={!canSelect}
        >
          {opcao}
        </button>
      {/each}
    </div>
  </div>

  <div class="action-buttons">
    <button class="btn btn-reveal" onclick={handleReveal} disabled={!canReveal}>
      👁️ Revelar Resposta
    </button>
    <button class="btn btn-next" onclick={handleNext} disabled={!canAdvance}>
      ➡️ Próxima Pergunta
    </button>
    <button class="btn btn-stop" onclick={handleStop} disabled={!canStop}>
      🛑 Parar
    </button>
  </div>
</div>

<style>
  .controls-panel {
    flex: 1;
    background: var(--color-bg-secondary);
    border-radius: var(--border-radius-lg);
    padding: var(--spacing-md);
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .controls-panel h3 {
    font-size: 1rem;
    color: var(--color-accent-gold);
    border-bottom: 1px solid var(--color-bg-card);
    padding-bottom: var(--spacing-sm);
  }

  .controls-label {
    font-size: 0.8rem;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .select-buttons {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .button-row {
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

  .btn-option {
    flex: 1;
    color: white;
    font-size: 1.1rem;
    padding: var(--spacing-sm);
  }

  .btn-option-a {
    background: var(--color-option-a);
  }
  .btn-option-b {
    background: var(--color-option-b);
  }
  .btn-option-c {
    background: var(--color-option-c);
  }
  .btn-option-d {
    background: var(--color-option-d);
  }

  .btn-option:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .action-buttons {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .btn-reveal {
    background: var(--color-accent-purple);
    color: white;
  }

  .btn-reveal:hover:not(:disabled) {
    background: #7b1fa2;
  }

  .btn-next {
    background: var(--color-accent-blue);
    color: white;
  }

  .btn-next:hover:not(:disabled) {
    background: #1976d2;
  }

  .btn-stop {
    background: var(--color-error);
    color: white;
  }

  .btn-stop:hover:not(:disabled) {
    background: #d32f2f;
  }
</style>
