<!-- Caminho relativo: src/display/QuestionDisplay.svelte -->
<!-- Exibição da pergunta e alternativas na tela do projetor -->
<script lang="ts">
  import type { GameState } from "../lib/types";
  import AnswerOption from "./AnswerOption.svelte";

  interface Props {
    gameState: GameState;
  }

  let { gameState }: Props = $props();

  let pergunta = $derived(gameState.perguntas[gameState.pergunta_atual]);

  let opcoes = $derived([
    { letra: "A", texto: pergunta.alternativas.A },
    { letra: "B", texto: pergunta.alternativas.B },
    { letra: "C", texto: pergunta.alternativas.C },
    { letra: "D", texto: pergunta.alternativas.D },
  ]);

  function getOptionState(
    letra: string,
  ): "normal" | "selected" | "correct" | "incorrect" | "eliminated" {
    const isCorreta = letra === pergunta.correta.toUpperCase();
    const isSelecionada = letra === gameState.resposta_selecionada;
    const isRevelada = gameState.status === "RespostaRevelada";
    const isEliminada =
      gameState.cartas_result &&
      !gameState.cartas_result.alternativas_visiveis.includes(letra);

    if (isEliminada) return "eliminated";
    if (isRevelada && isCorreta) return "correct";
    if (isRevelada && isSelecionada && !isCorreta) return "incorrect";
    if (isSelecionada) return "selected";
    return "normal";
  }
</script>

<div class="question-container">
  <div class="question-header">
    <span class="question-number">
      Pergunta {gameState.pergunta_atual + 1} de {gameState.config.premio.valores
        .length}
    </span>
    <span class="question-prize">
      Vale: {gameState.config.premio.icone}
      {gameState.config.premio.valores[gameState.pergunta_atual]?.toLocaleString(
        "pt-BR"
      )}
      {gameState.config.premio.nome}
    </span>
  </div>

  <div class="question-text">
    {pergunta.texto}
  </div>

  <div class="options-container">
    {#each opcoes as opcao}
      <AnswerOption
        letra={opcao.letra}
        texto={opcao.texto}
        state={getOptionState(opcao.letra)}
      />
    {/each}
  </div>
</div>

<style>
  .question-container {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .question-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .question-number {
    font-size: 1rem;
    color: var(--color-text-muted);
    font-weight: 600;
  }

  .question-prize {
    font-size: 1rem;
    color: var(--color-accent-gold);
    font-weight: 700;
    background: rgba(244, 160, 32, 0.1);
    padding: var(--spacing-xs) var(--spacing-md);
    border-radius: var(--border-radius-full);
    border: 1px solid rgba(244, 160, 32, 0.3);
  }

  .question-text {
    font-size: 1.75rem;
    font-weight: 800;
    line-height: 1.4;
    text-align: center;
    padding: var(--spacing-lg) var(--spacing-xl);
    background: var(--color-bg-secondary);
    border-radius: var(--border-radius-lg);
    border: 2px solid rgba(244, 160, 32, 0.15);
    box-shadow: var(--shadow-lg);
  }

  .options-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--spacing-md);
  }
</style>
