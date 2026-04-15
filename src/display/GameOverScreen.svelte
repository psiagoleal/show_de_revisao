<!-- Caminho relativo: src/display/GameOverScreen.svelte -->
<!-- Tela de fim de jogo exibida no projetor -->
<script lang="ts">
  import type { GameState } from "../lib/types";

  interface Props {
    gameState: GameState;
  }

  let { gameState }: Props = $props();

  let isVitoria = $derived(gameState.resultado_final === "Vitoria");
  let isDerrota = $derived(gameState.resultado_final === "Derrota");
  let isParou = $derived(gameState.resultado_final === "Parou");
  let isTimeout = $derived(gameState.resultado_final === "Timeout");

  let emoji = $derived(
    isVitoria ? "🏆" : isDerrota ? "😔" : isTimeout ? "⏱️" : "🛑"
  );

  let titulo = $derived(
    isVitoria
      ? "PARABÉNS! PRÊMIO MÁXIMO!"
      : isDerrota
        ? "QUE PENA! RESPOSTA INCORRETA!"
        : isTimeout
          ? "TEMPO ESGOTADO!"
          : "JOGO ENCERRADO!"
  );

  let subtitulo = $derived(
    isVitoria
      ? "Você conquistou o prêmio máximo!"
      : isDerrota
        ? "Você volta ao último checkpoint."
        : isTimeout
          ? "O tempo acabou!"
          : "Você decidiu parar e garantir o prêmio."
  );
</script>

<div class="game-over" class:vitoria={isVitoria} class:derrota={isDerrota}>
  <div class="game-over-content">
    <div class="result-emoji">{emoji}</div>
    <h1 class="result-title">{titulo}</h1>
    <p class="result-subtitle">{subtitulo}</p>

    <div class="final-prize-display">
      <span class="prize-label">Prêmio Final</span>
      <span class="prize-amount">
        {gameState.config.premio.icone}
        {gameState.premio_acumulado.toLocaleString("pt-BR")}
        {gameState.config.premio.nome}
      </span>
    </div>

    <div class="stats">
      <div class="stat">
        <span class="stat-value">{gameState.pergunta_atual + (isDerrota || isTimeout ? 1 : 0)}</span>
        <span class="stat-label">Perguntas respondidas</span>
      </div>
      <div class="stat">
        <span class="stat-value">
          {(gameState.ajudas_usadas.universitarios ? 1 : 0) +
            (gameState.ajudas_usadas.cartas ? 1 : 0) +
            gameState.ajudas_usadas.pulos_usados}
        </span>
        <span class="stat-label">Ajudas usadas</span>
      </div>
    </div>
  </div>
</div>

<style>
  .game-over {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: radial-gradient(
      ellipse at center,
      var(--color-bg-secondary) 0%,
      var(--color-bg-primary) 100%
    );
    animation: fadeIn 1s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .game-over-content {
    text-align: center;
    max-width: 600px;
  }

  .result-emoji {
    font-size: 6rem;
    margin-bottom: var(--spacing-lg);
    animation: bounceIn 0.8s ease;
  }

  @keyframes bounceIn {
    0% { transform: scale(0); }
    50% { transform: scale(1.2); }
    100% { transform: scale(1); }
  }

  .result-title {
    font-size: 2.5rem;
    font-weight: 900;
    color: var(--color-accent-gold);
    margin-bottom: var(--spacing-md);
    text-shadow: 0 2px 15px rgba(244, 160, 32, 0.3);
  }

  .vitoria .result-title {
    color: var(--color-success);
    text-shadow: 0 2px 15px rgba(76, 175, 80, 0.4);
  }

  .derrota .result-title {
    color: var(--color-error);
    text-shadow: 0 2px 15px rgba(244, 67, 54, 0.3);
  }

  .result-subtitle {
    font-size: 1.25rem;
    color: var(--color-text-secondary);
    margin-bottom: var(--spacing-2xl);
  }

  .final-prize-display {
    background: var(--color-bg-card);
    border-radius: var(--border-radius-lg);
    padding: var(--spacing-xl);
    margin-bottom: var(--spacing-xl);
    border: 2px solid rgba(244, 160, 32, 0.3);
  }

  .prize-label {
    display: block;
    font-size: 0.9rem;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 2px;
    margin-bottom: var(--spacing-sm);
  }

  .prize-amount {
    font-size: 3rem;
    font-weight: 900;
    color: var(--color-accent-gold);
    text-shadow: 0 2px 10px rgba(244, 160, 32, 0.3);
  }

  .stats {
    display: flex;
    gap: var(--spacing-xl);
    justify-content: center;
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-xs);
  }

  .stat-value {
    font-size: 2rem;
    font-weight: 900;
    color: var(--color-accent-blue);
  }

  .stat-label {
    font-size: 0.85rem;
    color: var(--color-text-muted);
  }
</style>
