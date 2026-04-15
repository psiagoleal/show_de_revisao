<!-- Caminho relativo: src/presenter/HelpControls.svelte -->
<!-- Controles das ajudas do jogo para o apresentador -->
<script lang="ts">
    import type { GameState } from "../lib/types";
    import {
        usarUniversitarios,
        usarCartas,
        usarPular,
        submeterVotos,
    } from "../lib/api";

    interface Props {
        gameState: GameState | null;
        onStateUpdate: (state: GameState) => void;
    }

    let { gameState, onStateUpdate }: Props = $props();

    let canUseHelp = $derived(gameState?.status === "MostrandoPergunta");

    let canUseUniversitarios = $derived(
        canUseHelp &&
            gameState?.config.ajudas.universitarios &&
            !gameState?.ajudas_usadas.universitarios,
    );

    let canUseCartas = $derived(
        canUseHelp &&
            gameState?.config.ajudas.cartas &&
            !gameState?.ajudas_usadas.cartas,
    );

    let canUsePular = $derived(
        canUseHelp &&
            gameState !== null &&
            gameState.ajudas_usadas.pulos_usados <
                gameState.config.ajudas.pular,
    );

    let pulosRestantes = $derived(
        gameState
            ? gameState.config.ajudas.pular -
                  gameState.ajudas_usadas.pulos_usados
            : 0,
    );

    let voteCounts: number[] = $state([0, 0, 0, 0]);

    async function handleUniversitarios() {
        try {
            const state = await usarUniversitarios();
            onStateUpdate(state);
        } catch (e: any) {
            console.error("Erro ao usar universitários:", e);
        }
    }

    async function handleSubmeterVotos() {
        try {
            const state = await submeterVotos(
                voteCounts[0],
                voteCounts[1],
                voteCounts[2],
                voteCounts[3],
            );
            onStateUpdate(state);
            voteCounts = [0, 0, 0, 0]; // reset for next use
        } catch (e: any) {
            console.error("Erro ao submeter votos:", e);
        }
    }

    async function handleCartas() {
        try {
            const state = await usarCartas();
            onStateUpdate(state);
        } catch (e: any) {
            console.error("Erro ao usar cartas:", e);
        }
    }

    async function handlePular() {
        try {
            const state = await usarPular();
            onStateUpdate(state);
        } catch (e: any) {
            console.error("Erro ao pular:", e);
        }
    }
</script>

<div class="help-panel">
    <h3>🆘 Ajudas</h3>

    <div class="help-buttons">
        {#if gameState?.votacao_em_andamento}
            <!-- Voting form -->
            <div class="voting-form">
                <h4>🎓 Votação dos Colegas</h4>
                <p class="voting-hint">
                    Conte as mãos levantadas para cada alternativa:
                </p>
                <div class="vote-inputs">
                    {#each ["A", "B", "C", "D"] as letra, i}
                        <div class="vote-input-group">
                            <label
                                for="vote-{letra.toLowerCase()}"
                                class="vote-label vote-label-{letra.toLowerCase()}"
                                >{letra}</label
                            >
                            <input
                                id="vote-{letra.toLowerCase()}"
                                type="number"
                                min="0"
                                class="vote-input"
                                bind:value={voteCounts[i]}
                                placeholder="0"
                            />
                        </div>
                    {/each}
                </div>
                <button
                    class="btn btn-confirm-votes"
                    onclick={handleSubmeterVotos}
                >
                    ✅ Confirmar Votos
                </button>
            </div>
        {:else}
            <!-- Original button, but now disabled if already used -->
            <button
                class="btn btn-help"
                onclick={handleUniversitarios}
                disabled={!canUseUniversitarios}
                class:used={gameState?.ajudas_usadas.universitarios}
            >
                <span class="help-icon">🎓</span>
                <span class="help-label">Universitários</span>
                {#if gameState?.ajudas_usadas.universitarios && !gameState?.votacao_em_andamento}
                    <span class="used-badge">Usada</span>
                {/if}
            </button>
        {/if}

        <button
            class="btn btn-help"
            onclick={handleCartas}
            disabled={!canUseCartas}
            class:used={gameState?.ajudas_usadas.cartas}
        >
            <span class="help-icon">🃏</span>
            <span class="help-label">Cartas</span>
            {#if gameState?.ajudas_usadas.cartas}
                <span class="used-badge">Usada</span>
            {/if}
        </button>

        <button
            class="btn btn-help"
            onclick={handlePular}
            disabled={!canUsePular}
            class:used={pulosRestantes === 0}
        >
            <span class="help-icon">⏭️</span>
            <span class="help-label">Pular ({pulosRestantes})</span>
            {#if pulosRestantes === 0}
                <span class="used-badge">Esgotado</span>
            {/if}
        </button>
    </div>
</div>

<style>
    .help-panel {
        flex: 1;
        background: var(--color-bg-secondary);
        border-radius: var(--border-radius-lg);
        padding: var(--spacing-md);
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md);
    }

    .help-panel h3 {
        font-size: 1rem;
        color: var(--color-accent-gold);
        border-bottom: 1px solid var(--color-bg-card);
        padding-bottom: var(--spacing-sm);
    }

    .help-buttons {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-sm);
    }

    .btn-help {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
        padding: var(--spacing-sm) var(--spacing-md);
        background: var(--color-bg-card);
        color: var(--color-text-primary);
        border-radius: var(--border-radius-md);
        font-weight: 600;
        transition: all var(--transition-fast);
    }

    .btn-help:hover:not(:disabled) {
        background: var(--color-bg-card-hover);
        transform: translateY(-1px);
    }

    .btn-help.used {
        opacity: 0.4;
    }

    .help-icon {
        font-size: 1.5rem;
    }

    .help-label {
        flex: 1;
        text-align: left;
    }

    .used-badge {
        font-size: 0.7rem;
        background: var(--color-error);
        color: white;
        padding: 2px 8px;
        border-radius: var(--border-radius-full);
        text-transform: uppercase;
        font-weight: 700;
    }

    .voting-form {
        background: var(--color-bg-card);
        border-radius: var(--border-radius-md);
        padding: var(--spacing-md);
        border: 2px solid var(--color-accent-purple);
    }

    .voting-form h4 {
        color: var(--color-accent-purple);
        margin-bottom: var(--spacing-xs);
        font-size: 0.95rem;
    }

    .voting-hint {
        font-size: 0.8rem;
        color: var(--color-text-muted);
        margin-bottom: var(--spacing-sm);
    }

    .vote-inputs {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: var(--spacing-sm);
        margin-bottom: var(--spacing-sm);
    }

    .vote-input-group {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
    }

    .vote-label {
        width: 30px;
        height: 30px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: var(--border-radius-full);
        font-weight: 800;
        font-size: 0.9rem;
        color: white;
        flex-shrink: 0;
    }

    .vote-label-a {
        background: var(--color-option-a);
    }
    .vote-label-b {
        background: var(--color-option-b);
    }
    .vote-label-c {
        background: var(--color-option-c);
    }
    .vote-label-d {
        background: var(--color-option-d);
    }

    .vote-input {
        flex: 1;
        padding: var(--spacing-xs) var(--spacing-sm);
        border-radius: var(--border-radius-sm);
        border: 1px solid var(--color-text-muted);
        background: var(--color-bg-primary);
        color: var(--color-text-primary);
        font-family: var(--font-family);
        font-size: 1rem;
        font-weight: 700;
        text-align: center;
        width: 60px;
    }

    .vote-input:focus {
        outline: none;
        border-color: var(--color-accent-purple);
    }

    .btn-confirm-votes {
        width: 100%;
        background: var(--color-accent-purple);
        color: white;
        padding: var(--spacing-sm);
        font-size: 0.9rem;
    }

    .btn-confirm-votes:hover:not(:disabled) {
        background: #7b1fa2;
    }
</style>
