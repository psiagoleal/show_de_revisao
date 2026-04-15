<!-- Caminho relativo: src/display/HelpStatus.svelte -->
<!-- Indicadores de ajudas disponíveis/usadas no display -->
<script lang="ts">
  import type { GameState } from "../lib/types";

  interface Props {
    gameState: GameState;
  }

  let { gameState }: Props = $props();

  let pulosRestantes = $derived(
    gameState.config.ajudas.pular - gameState.ajudas_usadas.pulos_usados
  );
</script>

<div class="help-status">
  {#if gameState.config.ajudas.universitarios}
    <div
      class="help-badge"
      class:used={gameState.ajudas_usadas.universitarios}
    >
      🎓
    </div>
  {/if}

  {#if gameState.config.ajudas.cartas}
    <div class="help-badge" class:used={gameState.ajudas_usadas.cartas}>
      🃏
    </div>
  {/if}

  {#if gameState.config.ajudas.pular > 0}
    {#each Array(gameState.config.ajudas.pular) as _, i}
      <div
        class="help-badge"
        class:used={i < gameState.ajudas_usadas.pulos_usados}
      >
        ⏭️
      </div>
    {/each}
  {/if}
</div>

<style>
  .help-status {
    display: flex;
    gap: var(--spacing-sm);
    align-items: center;
  }

  .help-badge {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    background: var(--color-bg-card);
    border-radius: var(--border-radius-full);
    border: 2px solid rgba(255, 255, 255, 0.1);
    transition: all var(--transition-normal);
  }

  .help-badge.used {
    opacity: 0.25;
    filter: grayscale(1);
    border-color: transparent;
  }
</style>
