<!-- Caminho relativo: src/presenter/PresenterQuestion.svelte -->
<!-- Exibe a pergunta atual com destaque na resposta correta (visível apenas para o apresentador) -->
<script lang="ts">
  import type { GameState } from "../lib/types";

  interface Props {
    gameState: GameState | null;
  }

  let { gameState }: Props = $props();

  let pergunta = $derived(
    gameState ? gameState.perguntas[gameState.pergunta_atual] : null
  );

  let opcoes = $derived(
    pergunta
      ? [
          { letra: "A", texto: pergunta.alternativas.A },
          { letra: "B", texto: pergunta.alternativas.B },
          { letra: "C", texto: pergunta.alternativas.C },
          { letra: "D", texto: pergunta.alternativas.D },
        ]
      : []
  );

  function getOptionClass(letra: string): string {
    if (!gameState || !pergunta) return "";

    const isCorreta = letra === pergunta.correta.toUpperCase();
    const isSelecionada = letra === gameState.resposta_selecionada;
    const isRevelada = gameState.status === "RespostaRevelada";
    const isEliminada =
      gameState.cartas_result &&
      !gameState.cartas_result.alternativas_visiveis.includes(letra);

    let classes: string[] = [];

    // Sempre mostrar a correta para o apresentador
    if (isCorreta) classes.push("correta-hint");
    if (isSelecionada) classes.push("selecionada");
    if (isRevelada && isCorreta) classes.push("revelada-correta");
    if (isRevelada && isSelecionada && !isCorreta) classes.push("revelada-errada");
    if (isEliminada) classes.push("eliminada");

    return classes.join(" ");
  }
</script>

{#if pergunta && gameState}
  <div class="question-presenter">
    <div class="question-number">
      Pergunta {gameState.pergunta_atual + 1} — Vale {gameState.config.premio.icone}
      {gameState.config.premio.valores[gameState.pergunta_atual]?.toLocaleString(
        "pt-BR"
      )}
      {gameState.config.premio.nome}
    </div>
    <div class="question-text">{pergunta.texto}</div>
    <div class="options-grid">
      {#each opcoes as opcao}
        <div class="option-item {getOptionClass(opcao.letra)}">
          <span class="option-letter">{opcao.letra}</span>
          <span class="option-text">{opcao.texto}</span>
          {#if opcao.letra === pergunta.correta.toUpperCase()}
            <span class="correct-badge">✓</span>
          {/if}
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .question-presenter {
    background: var(--color-bg-secondary);
    border-radius: var(--border-radius-lg);
    padding: var(--spacing-lg);
  }

  .question-number {
    font-size: 0.85rem;
    color: var(--color-text-muted);
    margin-bottom: var(--spacing-sm);
    font-weight: 600;
  }

  .question-text {
    font-size: 1.2rem;
    font-weight: 700;
    margin-bottom: var(--spacing-md);
    line-height: 1.5;
  }

  .options-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--spacing-sm);
  }

  .option-item {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    background: var(--color-bg-card);
    border-radius: var(--border-radius-md);
    border: 2px solid transparent;
    transition: all var(--transition-fast);
  }

  .option-letter {
    font-weight: 800;
    font-size: 1rem;
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--border-radius-full);
    background: var(--color-bg-primary);
    flex-shrink: 0;
  }

  .option-text {
    flex: 1;
    font-weight: 600;
  }

  .correct-badge {
    color: var(--color-success);
    font-weight: 900;
    font-size: 1.2rem;
  }

  /* Dica da correta - visível apenas para o apresentador */
  .correta-hint {
    border-color: rgba(76, 175, 80, 0.4);
    background: rgba(76, 175, 80, 0.08);
  }

  .selecionada {
    border-color: var(--color-accent-blue);
    background: rgba(33, 150, 243, 0.1);
  }

  .revelada-correta {
    border-color: var(--color-success);
    background: rgba(76, 175, 80, 0.2);
  }

  .revelada-errada {
    border-color: var(--color-error);
    background: rgba(244, 67, 54, 0.2);
  }

  .eliminada {
    opacity: 0.3;
    text-decoration: line-through;
  }
</style>
