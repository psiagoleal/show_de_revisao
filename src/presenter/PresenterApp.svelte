<!-- Caminho relativo: src/presenter/PresenterApp.svelte -->
<!-- Componente principal da janela do Apresentador -->
<script lang="ts">
    import "../app.css";
    import { carregarJogo, iniciarJogo, reiniciarJogo } from "../lib/api";
    import type { GameState } from "../lib/types";
    import PresenterControls from "./PresenterControls.svelte";
    import PresenterQuestion from "./PresenterQuestion.svelte";
    import HelpControls from "./HelpControls.svelte";
    import TimerControl from "./TimerControl.svelte";

    let gameState: GameState | null = $state(null);
    let error: string | null = $state(null);
    let loading: boolean = $state(false);

    let isGameLoaded = $derived(gameState !== null);
    let isGameStarted = $derived(
        gameState !== null && gameState.status !== "AguardandoInicio",
    );
    let isGameFinished = $derived(gameState?.status === "Finalizado");

    async function handleCarregar() {
        loading = true;
        error = null;
        try {
            gameState = await carregarJogo();
        } catch (e: any) {
            error = e.toString();
        } finally {
            loading = false;
        }
    }

    async function handleIniciar() {
        error = null;
        try {
            gameState = await iniciarJogo();
        } catch (e: any) {
            error = e.toString();
        }
    }

    async function handleReiniciar() {
        error = null;
        try {
            gameState = await reiniciarJogo();
        } catch (e: any) {
            error = e.toString();
        }
    }

    function updateState(newState: GameState) {
        gameState = newState;
    }
</script>

<main class="presenter">
    <header class="presenter-header">
        <h1>🎯 {gameState?.config.titulo ?? "Show de Revisão"}</h1>
        <span class="subtitle">Painel do Apresentador</span>
    </header>

    {#if error}
        <div class="error-banner">
            <span>⚠️ {error}</span>
            <button onclick={() => (error = null)}>✕</button>
        </div>
    {/if}

    {#if !isGameLoaded}
        <div class="setup-screen">
            <div class="setup-card">
                <h2>🎮 Preparar Jogo</h2>
                <p>
                    Certifique-se de que os arquivos <code>config.json</code> e
                    <code>perguntas.json</code> estão na mesma pasta do programa.
                </p>
                <button
                    class="btn btn-primary btn-large"
                    onclick={handleCarregar}
                    disabled={loading}
                >
                    {loading ? "Carregando..." : "📂 Carregar Jogo"}
                </button>
            </div>
        </div>
    {:else if !isGameStarted}
        <div class="setup-screen">
            <div class="setup-card">
                <h2>✅ Jogo Carregado!</h2>
                <div class="game-info">
                    <p>
                        <strong>Perguntas no banco:</strong>
                        {gameState!.total_pool}
                    </p>
                    <p>
                        <strong>Perguntas por partida:</strong>
                        {gameState!.config.rodadas}
                    </p>
                    <p>
                        <strong>Prêmio:</strong>
                        {gameState!.config.premio.icone}
                        {gameState!.config.premio.nome}
                    </p>
                    <p>
                        <strong>Prêmio máximo:</strong>
                        {gameState!.config.premio.valores[
                            gameState!.config.premio.valores.length - 1
                        ].toLocaleString("pt-BR")}
                        {gameState!.config.premio.nome}
                    </p>
                </div>
                <button
                    class="btn btn-primary btn-large"
                    onclick={handleIniciar}
                >
                    🚀 Iniciar Jogo
                </button>
            </div>
        </div>
    {:else}
        <div class="game-layout">
            <div class="game-main">
                <div class="game-status-bar">
                    <div class="status-item">
                        <span class="status-label">Pergunta</span>
                        <span class="status-value">
                            {gameState!.pergunta_atual + 1} / {gameState!.config
                                .premio.valores.length}
                        </span>
                    </div>
                    <div class="status-item">
                        <span class="status-label">Prêmio Atual</span>
                        <span class="status-value highlight">
                            {gameState!.config.premio.icone}
                            {gameState!.premio_acumulado.toLocaleString(
                                "pt-BR",
                            )}
                            {gameState!.config.premio.nome}
                        </span>
                    </div>
                    <div class="status-item">
                        <span class="status-label">Prêmio Garantido</span>
                        <span class="status-value">
                            {gameState!.premio_garantido.toLocaleString(
                                "pt-BR",
                            )}
                            {gameState!.config.premio.nome}
                        </span>
                    </div>
                    <div class="status-item">
                        <span class="status-label">Status</span>
                        <span class="status-value status-badge"
                            >{gameState!.status}</span
                        >
                    </div>
                    <div class="status-item">
                        <span class="status-label">Banco de Perguntas</span>
                        <span class="status-value">
                            {gameState!.pool_restante} restantes de {gameState!
                                .total_pool}
                        </span>
                    </div>
                </div>

                {#if !isGameFinished}
                    <PresenterQuestion {gameState} />

                    <div class="controls-row">
                        <PresenterControls
                            {gameState}
                            onStateUpdate={updateState}
                        />
                        <HelpControls {gameState} onStateUpdate={updateState} />
                    </div>

                    <TimerControl {gameState} onStateUpdate={updateState} />
                {:else}
                    <div class="game-over-presenter">
                        <h2>
                            {#if gameState!.resultado_final === "Vitoria"}
                                🏆 VITÓRIA! Prêmio máximo conquistado!
                            {:else if gameState!.resultado_final === "Derrota"}
                                ❌ Resposta incorreta!
                            {:else if gameState!.resultado_final === "Parou"}
                                🛑 Jogo encerrado pelo participante
                            {:else if gameState!.resultado_final === "Timeout"}
                                ⏱️ Tempo esgotado!
                            {/if}
                        </h2>
                        <p class="final-prize">
                            Prêmio final: {gameState!.config.premio.icone}
                            <strong>
                                {gameState!.premio_acumulado.toLocaleString(
                                    "pt-BR",
                                )}
                                {gameState!.config.premio.nome}
                            </strong>
                        </p>
                        {#if gameState!.pool_resetado}
                            <p class="pool-reset-notice">
                                ℹ️ O banco de perguntas foi resetado — todas as
                                perguntas estão disponíveis novamente.
                            </p>
                        {/if}
                        <button
                            class="btn btn-primary btn-large"
                            onclick={handleReiniciar}
                        >
                            🔄 Nova Partida
                        </button>
                    </div>
                {/if}
            </div>
        </div>
    {/if}
</main>

<style>
    .presenter {
        height: 100%;
        display: flex;
        flex-direction: column;
        background: var(--color-bg-primary);
        padding: var(--spacing-md);
        overflow-y: auto;
    }

    .presenter-header {
        display: flex;
        align-items: center;
        gap: var(--spacing-md);
        padding-bottom: var(--spacing-md);
        border-bottom: 2px solid var(--color-bg-card);
        margin-bottom: var(--spacing-md);
    }

    .presenter-header h1 {
        font-size: 1.5rem;
        font-weight: 800;
        color: var(--color-accent-gold);
    }

    .subtitle {
        font-size: 0.875rem;
        color: var(--color-text-muted);
        background: var(--color-bg-secondary);
        padding: var(--spacing-xs) var(--spacing-sm);
        border-radius: var(--border-radius-sm);
    }

    .error-banner {
        display: flex;
        align-items: center;
        justify-content: space-between;
        background: rgba(244, 67, 54, 0.15);
        border: 1px solid var(--color-error);
        color: var(--color-error);
        padding: var(--spacing-sm) var(--spacing-md);
        border-radius: var(--border-radius-md);
        margin-bottom: var(--spacing-md);
    }

    .error-banner button {
        background: none;
        color: var(--color-error);
        font-size: 1.25rem;
        padding: var(--spacing-xs);
    }

    .setup-screen {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .setup-card {
        background: var(--color-bg-secondary);
        border-radius: var(--border-radius-lg);
        padding: var(--spacing-2xl);
        text-align: center;
        max-width: 500px;
        box-shadow: var(--shadow-lg);
    }

    .setup-card h2 {
        font-size: 1.75rem;
        margin-bottom: var(--spacing-md);
        color: var(--color-accent-gold);
    }

    .setup-card p {
        color: var(--color-text-secondary);
        margin-bottom: var(--spacing-lg);
        line-height: 1.6;
    }

    .setup-card code {
        background: var(--color-bg-card);
        padding: 2px 6px;
        border-radius: var(--border-radius-sm);
        font-size: 0.9em;
        color: var(--color-accent-blue);
    }

    .game-info {
        text-align: left;
        background: var(--color-bg-card);
        padding: var(--spacing-md);
        border-radius: var(--border-radius-md);
        margin-bottom: var(--spacing-lg);
    }

    .game-info p {
        margin-bottom: var(--spacing-xs);
        color: var(--color-text-primary);
    }

    .btn {
        padding: var(--spacing-sm) var(--spacing-lg);
        border-radius: var(--border-radius-md);
        font-weight: 700;
        font-size: 1rem;
        transition: all var(--transition-fast);
    }

    .btn-primary {
        background: var(--color-accent-gold);
        color: var(--color-bg-primary);
    }

    .btn-primary:hover:not(:disabled) {
        background: #e6941a;
        transform: translateY(-1px);
    }

    .btn-large {
        padding: var(--spacing-md) var(--spacing-xl);
        font-size: 1.125rem;
    }

    .game-layout {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md);
        overflow-y: auto;
    }

    .game-main {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: var(--spacing-md);
    }

    .game-status-bar {
        display: flex;
        gap: var(--spacing-md);
        flex-wrap: wrap;
    }

    .status-item {
        flex: 1;
        min-width: 150px;
        background: var(--color-bg-secondary);
        padding: var(--spacing-sm) var(--spacing-md);
        border-radius: var(--border-radius-md);
        display: flex;
        flex-direction: column;
        gap: var(--spacing-xs);
    }

    .status-label {
        font-size: 0.75rem;
        color: var(--color-text-muted);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .status-value {
        font-size: 1rem;
        font-weight: 700;
    }

    .status-value.highlight {
        color: var(--color-accent-gold);
    }

    .status-badge {
        font-size: 0.8rem;
        color: var(--color-accent-blue);
    }

    .controls-row {
        display: flex;
        gap: var(--spacing-md);
        flex-wrap: wrap;
    }

    .game-over-presenter {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: var(--spacing-lg);
        text-align: center;
    }

    .game-over-presenter h2 {
        font-size: 1.75rem;
        color: var(--color-accent-gold);
    }

    .final-prize {
        font-size: 1.25rem;
        color: var(--color-text-secondary);
    }

    .final-prize strong {
        color: var(--color-accent-gold);
        font-size: 1.5rem;
    }

    .pool-reset-notice {
        font-size: 0.9rem;
        color: var(--color-warning);
        background: rgba(255, 152, 0, 0.1);
        padding: var(--spacing-sm) var(--spacing-md);
        border-radius: var(--border-radius-md);
        border: 1px solid rgba(255, 152, 0, 0.3);
        text-align: center;
    }
</style>
