<!-- Caminho relativo: src/display/DisplayApp.svelte -->
<!-- Componente principal da janela de Apresentação (projetor) -->
<script lang="ts">
    import "../app.css";
    import { onEstadoAtualizado } from "../lib/events";
    import type { GameState } from "../lib/types";
    import QuestionDisplay from "./QuestionDisplay.svelte";
    import PrizeLadder from "./PrizeLadder.svelte";
    import HelpStatus from "./HelpStatus.svelte";
    import TimerDisplay from "./TimerDisplay.svelte";
    import UniversityHelp from "./UniversityHelp.svelte";
    import GameOverScreen from "./GameOverScreen.svelte";

    let gameState: GameState | null = $state(null);

    let isWaiting = $derived(
        !gameState || gameState.status === "AguardandoInicio",
    );
    let isGameFinished = $derived(gameState?.status === "Finalizado");
    let showUniversityHelp = $derived(
        gameState?.university_votes !== null &&
            gameState?.university_votes !== undefined,
    );
    let showVotingInProgress = $derived(
        gameState?.votacao_em_andamento === true,
    );

    // Escutar eventos do backend
    $effect(() => {
        let unlisten: (() => void) | undefined;

        onEstadoAtualizado((state) => {
            gameState = state;
        }).then((fn) => {
            unlisten = fn;
        });

        return () => {
            if (unlisten) unlisten();
        };
    });
</script>

<main class="display">
    {#if isWaiting}
        <div class="waiting-screen">
            <div class="waiting-content">
                <div class="logo-icon">🎯</div>
                <h1>{gameState?.config.titulo ?? "Show de Questão"}</h1>
                <p>Aguardando o apresentador iniciar o jogo...</p>
                <div class="loading-dots">
                    <span class="dot"></span>
                    <span class="dot"></span>
                    <span class="dot"></span>
                </div>
            </div>
        </div>
    {:else if isGameFinished && gameState}
        <GameOverScreen {gameState} />
    {:else if gameState}
        <div class="game-display">
            <div class="display-header">
                <h1 class="game-title">{gameState.config.titulo}</h1>
                <HelpStatus {gameState} />
            </div>

            <div class="display-content">
                <div class="question-area">
                    {#if showVotingInProgress}
                        <div class="voting-in-progress">
                            <div class="voting-icon">🗳️</div>
                            <h2>Votação dos Colegas</h2>
                            <p>
                                Levante a mão para a alternativa que você
                                acredita ser a correta!
                            </p>
                            <div class="voting-dots">
                                <span class="dot"></span>
                                <span class="dot"></span>
                                <span class="dot"></span>
                            </div>
                        </div>
                    {/if}

                    {#if showUniversityHelp}
                        <UniversityHelp votes={gameState.university_votes!} />
                    {/if}

                    <QuestionDisplay {gameState} />

                    {#if gameState.timer_segundos_restantes !== null && gameState.timer_segundos_restantes !== undefined}
                        <TimerDisplay
                            segundos={gameState.timer_segundos_restantes}
                            ativo={gameState.timer_ativo}
                        />
                    {/if}
                </div>

                <div class="ladder-area">
                    <PrizeLadder {gameState} />
                </div>
            </div>
        </div>
    {/if}
</main>

<style>
    .display {
        height: 100%;
        background: linear-gradient(
            135deg,
            #0d1b2a 0%,
            #1b2838 50%,
            #0d1b2a 100%
        );
        overflow: hidden;
    }

    /* Waiting Screen */
    .waiting-screen {
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .waiting-content {
        text-align: center;
        animation: fadeIn 1s ease;
    }

    .logo-icon {
        font-size: 5rem;
        margin-bottom: var(--spacing-lg);
        animation: float 3s ease-in-out infinite;
    }

    @keyframes float {
        0%,
        100% {
            transform: translateY(0);
        }
        50% {
            transform: translateY(-15px);
        }
    }

    .waiting-content h1 {
        font-size: 3rem;
        font-weight: 900;
        color: var(--color-accent-gold);
        margin-bottom: var(--spacing-md);
        text-shadow: 0 2px 10px rgba(244, 160, 32, 0.3);
    }

    .waiting-content p {
        font-size: 1.25rem;
        color: var(--color-text-secondary);
        margin-bottom: var(--spacing-xl);
    }

    .loading-dots {
        display: flex;
        gap: var(--spacing-sm);
        justify-content: center;
    }

    .dot {
        width: 12px;
        height: 12px;
        border-radius: 50%;
        background: var(--color-accent-gold);
        animation: bounce 1.4s infinite ease-in-out;
    }

    .dot:nth-child(2) {
        animation-delay: 0.2s;
    }
    .dot:nth-child(3) {
        animation-delay: 0.4s;
    }

    @keyframes bounce {
        0%,
        80%,
        100% {
            transform: scale(0.6);
            opacity: 0.4;
        }
        40% {
            transform: scale(1);
            opacity: 1;
        }
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    /* Game Display */
    .game-display {
        height: 100%;
        display: flex;
        flex-direction: column;
        padding: var(--spacing-md) var(--spacing-lg);
    }

    .display-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-bottom: var(--spacing-md);
        border-bottom: 2px solid rgba(244, 160, 32, 0.2);
        margin-bottom: var(--spacing-md);
    }

    .game-title {
        font-size: 1.5rem;
        font-weight: 900;
        color: var(--color-accent-gold);
        text-shadow: 0 2px 8px rgba(244, 160, 32, 0.2);
    }

    .display-content {
        flex: 1;
        display: flex;
        gap: var(--spacing-lg);
        min-height: 0;
    }

    .question-area {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md);
        justify-content: center;
    }

    .ladder-area {
        width: 280px;
        flex-shrink: 0;
    }

    /* Voting in progress */
    .voting-in-progress {
        background: var(--color-bg-secondary);
        border-radius: var(--border-radius-lg);
        padding: var(--spacing-xl);
        text-align: center;
        border: 2px solid rgba(156, 39, 176, 0.3);
        animation: fadeIn 0.5s ease;
    }

    .voting-icon {
        font-size: 3rem;
        margin-bottom: var(--spacing-md);
        animation: float 3s ease-in-out infinite;
    }

    .voting-in-progress h2 {
        color: var(--color-accent-purple);
        font-size: 1.5rem;
        margin-bottom: var(--spacing-sm);
    }

    .voting-in-progress p {
        color: var(--color-text-secondary);
        font-size: 1.1rem;
        margin-bottom: var(--spacing-md);
    }

    .voting-dots {
        display: flex;
        gap: var(--spacing-sm);
        justify-content: center;
    }

    .voting-dots .dot {
        width: 10px;
        height: 10px;
        border-radius: 50%;
        background: var(--color-accent-purple);
        animation: bounce 1.4s infinite ease-in-out;
    }

    .voting-dots .dot:nth-child(2) {
        animation-delay: 0.2s;
    }
    .voting-dots .dot:nth-child(3) {
        animation-delay: 0.4s;
    }
</style>
