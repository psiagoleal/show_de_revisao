<!-- Caminho relativo: src/presenter/PresenterApp.svelte -->
<!-- Componente principal da janela do Apresentador -->
<script lang="ts">
    import "../app.css";
    import { open } from "@tauri-apps/plugin-dialog";
    import {
        carregarJogo,
        iniciarJogo,
        reiniciarJogo,
        obterEstado,
        resetarHistorico,
    } from "../lib/api";
    import type { GameState } from "../lib/types";
    import PresenterControls from "./PresenterControls.svelte";
    import PresenterQuestion from "./PresenterQuestion.svelte";
    import HelpControls from "./HelpControls.svelte";
    import TimerControl from "./TimerControl.svelte";

    let gameState: GameState | null = $state(null);
    let error: string | null = $state(null);
    let loading: boolean = $state(false);
    let loadingReset: boolean = $state(false);

    /** Caminho absoluto do arquivo de perguntas selecionado via dialog.
     *  String vazia = usar padrão (perguntas.json ao lado do executável). */
    let selectedFilePath: string = $state("");

    let isGameLoaded = $derived(gameState !== null);
    let isGameStarted = $derived(
        gameState !== null && gameState.status !== "AguardandoInicio",
    );
    let isGameFinished = $derived(gameState?.status === "Finalizado");

    /** Nome curto do arquivo selecionado (sem o path completo) */
    let nomeArquivoSelecionado = $derived(
        selectedFilePath
            ? (selectedFilePath.split(/[\\/]/).pop() ?? selectedFilePath)
            : "perguntas.json (padrão)",
    );

    /** Nome curto do arquivo carregado no GameState */
    let nomeArquivoCarregado = $derived(
        gameState?.arquivo_perguntas
            ? (gameState.arquivo_perguntas.split(/[\\/]/).pop() ??
                  gameState.arquivo_perguntas)
            : "—",
    );

    /** Quantas perguntas já foram realizadas no banco atual */
    let perguntasUsadas = $derived(
        gameState ? gameState.total_pool - gameState.pool_restante : 0,
    );

    /** Aviso quando o pool está prestes a resetar na próxima partida */
    let avisoPoolBaixo = $derived(
        gameState !== null &&
            gameState.pool_restante < gameState.config.rodadas &&
            gameState.pool_restante > 0,
    );

    /** Pool totalmente esgotado — vai resetar automaticamente */
    let poolEsgotado = $derived(
        gameState !== null && gameState.pool_restante === 0,
    );

    /** Abre o dialog nativo para o apresentador escolher o arquivo de perguntas */
    async function handleSelecionarArquivo() {
        try {
            const selected = await open({
                multiple: false,
                filters: [
                    { name: "Banco de Perguntas JSON", extensions: ["json"] },
                ],
                title: "Selecionar banco de perguntas",
            });
            if (selected && typeof selected === "string") {
                selectedFilePath = selected;
                // Limpa o gameState anterior ao trocar o arquivo
                gameState = null;
            }
        } catch (e: any) {
            error = `Erro ao abrir seletor de arquivo: ${e}`;
        }
    }

    /** Carrega o jogo com o arquivo selecionado (ou o padrão) */
    async function handleCarregar() {
        loading = true;
        error = null;
        try {
            gameState = await carregarJogo(selectedFilePath);
        } catch (e: any) {
            error = e.toString();
        } finally {
            loading = false;
        }
    }

    /** Volta para a tela de seleção de arquivo */
    function handleVoltar() {
        gameState = null;
        error = null;
    }

    /** Reseta o histórico do banco atual e atualiza o estado */
    async function handleReset() {
        if (!gameState) return;
        loadingReset = true;
        error = null;
        try {
            await resetarHistorico(gameState.arquivo_perguntas);
            // Recarrega o estado atualizado
            const estado = await obterEstado();
            if (estado) gameState = estado;
        } catch (e: any) {
            error = e.toString();
        } finally {
            loadingReset = false;
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
        <!-- ── Tela de seleção de arquivo ── -->
        <div class="setup-screen">
            <div class="setup-card">
                <h2>🎮 Preparar Jogo</h2>
                <p class="setup-hint">
                    Escolha o arquivo de banco de perguntas ou use o arquivo
                    padrão <code>perguntas.json</code> ao lado do programa. O
                    arquivo <code>config.json</code> deve estar na mesma pasta do
                    executável.
                </p>

                <div class="file-selector">
                    <div class="file-display">
                        <span class="file-icon">📄</span>
                        <span
                            class="file-name"
                            title={selectedFilePath || "Arquivo padrão"}
                        >
                            {nomeArquivoSelecionado}
                        </span>
                    </div>
                    <button
                        class="btn btn-secondary"
                        onclick={handleSelecionarArquivo}
                        title="Abrir explorador para escolher o arquivo JSON de perguntas"
                    >
                        📂 Escolher arquivo...
                    </button>
                </div>

                <button
                    class="btn btn-primary btn-large"
                    onclick={handleCarregar}
                    disabled={loading}
                >
                    {loading ? "⏳ Carregando..." : "✅ Carregar Jogo"}
                </button>
            </div>
        </div>
    {:else if !isGameStarted}
        <!-- ── Tela de jogo carregado (antes de iniciar) ── -->
        <div class="setup-screen">
            <div class="setup-card">
                <h2>✅ Jogo Carregado!</h2>

                <div class="game-info">
                    <div class="info-row">
                        <span class="info-label">📄 Arquivo</span>
                        <span
                            class="info-value file-name-small"
                            title={gameState!.arquivo_perguntas}
                        >
                            {nomeArquivoCarregado}
                        </span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">❓ Perguntas no banco</span>
                        <span class="info-value">{gameState!.total_pool}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">🎯 Perguntas por partida</span>
                        <span class="info-value"
                            >{gameState!.config.rodadas}</span
                        >
                    </div>
                    <div class="info-row">
                        <span class="info-label">🏆 Prêmio máximo</span>
                        <span class="info-value">
                            {gameState!.config.premio.icone}
                            {gameState!.config.premio.valores[
                                gameState!.config.premio.valores.length - 1
                            ].toLocaleString("pt-BR")}
                            {gameState!.config.premio.nome}
                        </span>
                    </div>
                </div>

                <!-- Painel de histórico do banco de perguntas -->
                <div class="history-panel">
                    <h3 class="history-title">📊 Histórico do Banco</h3>
                    <div class="history-stats">
                        <div class="history-stat">
                            <span class="stat-number stat-used"
                                >{perguntasUsadas}</span
                            >
                            <span class="stat-label">realizadas</span>
                        </div>
                        <div class="history-divider">/</div>
                        <div class="history-stat">
                            <span class="stat-number stat-total"
                                >{gameState!.total_pool}</span
                            >
                            <span class="stat-label">total</span>
                        </div>
                        <div class="history-divider">·</div>
                        <div class="history-stat">
                            <span class="stat-number stat-available"
                                >{gameState!.pool_restante}</span
                            >
                            <span class="stat-label">disponíveis</span>
                        </div>
                    </div>

                    <!-- Barra de progresso do pool -->
                    <div
                        class="pool-bar"
                        title="{perguntasUsadas} de {gameState!
                            .total_pool} perguntas usadas"
                    >
                        <div
                            class="pool-bar-fill"
                            style="width: {Math.round(
                                (perguntasUsadas / gameState!.total_pool) * 100,
                            )}%"
                        ></div>
                    </div>

                    {#if poolEsgotado}
                        <p class="pool-notice pool-notice--warn">
                            ⚠️ Todas as perguntas já foram realizadas. O banco
                            será <strong>resetado automaticamente</strong> ao iniciar
                            esta partida.
                        </p>
                    {:else if avisoPoolBaixo}
                        <p class="pool-notice pool-notice--info">
                            ℹ️ Restam apenas {gameState!.pool_restante} perguntas
                            disponíveis (menos que as {gameState!.config
                                .rodadas}
                            necessárias). O banco será
                            <strong>resetado automaticamente</strong> ao iniciar.
                        </p>
                    {:else if perguntasUsadas === 0}
                        <p class="pool-notice pool-notice--ok">
                            ✅ Banco completo — nenhuma pergunta foi realizada
                            ainda.
                        </p>
                    {/if}

                    <button
                        class="btn btn-danger btn-sm"
                        onclick={handleReset}
                        disabled={loadingReset || perguntasUsadas === 0}
                        title="Apaga o histórico e permite que todas as perguntas sejam sorteadas novamente"
                    >
                        {loadingReset
                            ? "⏳ Resetando..."
                            : "🔄 Resetar histórico deste banco"}
                    </button>
                </div>

                <div class="action-buttons">
                    <button class="btn btn-ghost" onclick={handleVoltar}>
                        ↩ Trocar arquivo
                    </button>
                    <button
                        class="btn btn-primary btn-large"
                        onclick={handleIniciar}
                    >
                        🚀 Iniciar Jogo
                    </button>
                </div>
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

    /* ── Setup: hint e seletor de arquivo ── */
    .setup-hint {
        color: var(--color-text-secondary);
        line-height: 1.6;
        margin-bottom: var(--spacing-lg);
    }

    .file-selector {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
        background: var(--color-bg-card);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: var(--border-radius-md);
        padding: var(--spacing-sm) var(--spacing-md);
        margin-bottom: var(--spacing-lg);
    }

    .file-display {
        display: flex;
        align-items: center;
        gap: var(--spacing-xs);
        flex: 1;
        min-width: 0;
    }

    .file-icon {
        font-size: 1.25rem;
        flex-shrink: 0;
    }

    .file-name {
        font-size: 0.9rem;
        color: var(--color-accent-blue);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        font-family: monospace;
    }

    .file-name-small {
        font-size: 0.85rem;
        color: var(--color-accent-blue);
        font-family: monospace;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 200px;
        display: inline-block;
        vertical-align: middle;
    }

    .btn-secondary {
        background: var(--color-bg-primary);
        border: 1px solid rgba(255, 255, 255, 0.2);
        color: var(--color-text-primary);
        flex-shrink: 0;
    }

    .btn-secondary:hover {
        background: var(--color-bg-card);
        border-color: var(--color-accent-blue);
    }

    /* ── Game info rows ── */
    .info-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: var(--spacing-xs) 0;
        border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    }

    .info-row:last-child {
        border-bottom: none;
    }

    .info-label {
        font-size: 0.85rem;
        color: var(--color-text-muted);
    }

    .info-value {
        font-size: 0.95rem;
        font-weight: 600;
        color: var(--color-text-primary);
        text-align: right;
    }

    /* ── Painel de histórico ── */
    .history-panel {
        background: var(--color-bg-card);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: var(--border-radius-md);
        padding: var(--spacing-md);
        margin-top: var(--spacing-md);
        margin-bottom: var(--spacing-md);
        display: flex;
        flex-direction: column;
        gap: var(--spacing-sm);
    }

    .history-title {
        font-size: 0.9rem;
        font-weight: 700;
        color: var(--color-text-muted);
        text-transform: uppercase;
        letter-spacing: 0.5px;
        margin: 0;
    }

    .history-stats {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: var(--spacing-sm);
    }

    .history-stat {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 2px;
    }

    .stat-number {
        font-size: 1.5rem;
        font-weight: 800;
        line-height: 1;
    }

    .stat-label {
        font-size: 0.7rem;
        color: var(--color-text-muted);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .stat-used {
        color: var(--color-warning);
    }
    .stat-total {
        color: var(--color-text-secondary);
    }
    .stat-available {
        color: var(--color-correct, #4caf50);
    }

    .history-divider {
        font-size: 1.25rem;
        color: var(--color-text-muted);
        padding: 0 var(--spacing-xs);
        align-self: center;
    }

    /* Barra de progresso do pool */
    .pool-bar {
        height: 6px;
        background: rgba(255, 255, 255, 0.08);
        border-radius: 3px;
        overflow: hidden;
    }

    .pool-bar-fill {
        height: 100%;
        background: linear-gradient(90deg, var(--color-warning), #f44336);
        border-radius: 3px;
        transition: width 0.4s ease;
        min-width: 0%;
        max-width: 100%;
    }

    /* Avisos do pool */
    .pool-notice {
        font-size: 0.82rem;
        padding: var(--spacing-xs) var(--spacing-sm);
        border-radius: var(--border-radius-sm);
        line-height: 1.5;
        margin: 0;
    }

    .pool-notice--ok {
        color: #4caf50;
        background: rgba(76, 175, 80, 0.1);
        border: 1px solid rgba(76, 175, 80, 0.25);
    }

    .pool-notice--info {
        color: var(--color-accent-blue);
        background: rgba(66, 165, 245, 0.1);
        border: 1px solid rgba(66, 165, 245, 0.25);
    }

    .pool-notice--warn {
        color: var(--color-warning);
        background: rgba(255, 152, 0, 0.1);
        border: 1px solid rgba(255, 152, 0, 0.3);
    }

    /* Botão de reset do histórico */
    .btn-danger {
        background: rgba(244, 67, 54, 0.12);
        color: #ef5350;
        border: 1px solid rgba(244, 67, 54, 0.3);
        font-size: 0.85rem;
    }

    .btn-danger:hover:not(:disabled) {
        background: rgba(244, 67, 54, 0.22);
        border-color: rgba(244, 67, 54, 0.6);
    }

    .btn-danger:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .btn-sm {
        padding: var(--spacing-xs) var(--spacing-md);
        font-size: 0.85rem;
    }

    /* Rodapé da card com ações */
    .action-buttons {
        display: flex;
        gap: var(--spacing-md);
        justify-content: center;
        align-items: center;
        margin-top: var(--spacing-sm);
        flex-wrap: wrap;
    }

    .btn-ghost {
        background: transparent;
        color: var(--color-text-muted);
        border: 1px solid rgba(255, 255, 255, 0.15);
        font-size: 0.9rem;
    }

    .btn-ghost:hover {
        background: var(--color-bg-card);
        color: var(--color-text-primary);
        border-color: rgba(255, 255, 255, 0.3);
    }
</style>
