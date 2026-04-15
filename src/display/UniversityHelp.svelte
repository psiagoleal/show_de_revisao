<!-- Caminho relativo: src/display/UniversityHelp.svelte -->
<!-- Gráfico de barras da ajuda dos universitários -->
<script lang="ts">
    import type { UniversityVotes } from "../lib/types";

    interface Props {
        votes: UniversityVotes;
    }

    let { votes }: Props = $props();

    let bars = $derived([
        { letra: "A", pct: votes.a, color: "var(--color-option-a)" },
        { letra: "B", pct: votes.b, color: "var(--color-option-b)" },
        { letra: "C", pct: votes.c, color: "var(--color-option-c)" },
        { letra: "D", pct: votes.d, color: "var(--color-option-d)" },
    ]);
</script>

<div class="university-help">
    <div class="university-title">
        <span>🎓</span> Votação dos Colegas
    </div>
    <div class="bars-container">
        {#each bars as bar}
            <div class="bar-wrapper">
                <div class="bar-label">{bar.letra}</div>
                <div class="bar-track">
                    <div
                        class="bar-fill"
                        style="width: {bar.pct}%; background: {bar.color}"
                    ></div>
                </div>
                <div class="bar-pct">{bar.pct}%</div>
            </div>
        {/each}
    </div>
    {#if votes.total_votos}
        <div class="total-votes">
            Total: {votes.total_votos} votos
        </div>
    {/if}
</div>

<style>
    .university-help {
        background: var(--color-bg-secondary);
        border-radius: var(--border-radius-lg);
        padding: var(--spacing-md) var(--spacing-lg);
        border: 2px solid rgba(156, 39, 176, 0.3);
        animation: slideDown 0.5s ease;
    }

    @keyframes slideDown {
        from {
            opacity: 0;
            transform: translateY(-20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .university-title {
        font-size: 1.1rem;
        font-weight: 800;
        color: var(--color-accent-purple);
        margin-bottom: var(--spacing-md);
        text-align: center;
    }

    .bars-container {
        display: flex;
        flex-direction: column;
        gap: var(--spacing-sm);
    }

    .bar-wrapper {
        display: flex;
        align-items: center;
        gap: var(--spacing-sm);
    }

    .bar-label {
        width: 30px;
        font-weight: 800;
        font-size: 1rem;
        text-align: center;
    }

    .bar-track {
        flex: 1;
        height: 28px;
        background: var(--color-bg-primary);
        border-radius: var(--border-radius-full);
        overflow: hidden;
    }

    .bar-fill {
        height: 100%;
        border-radius: var(--border-radius-full);
        transition: width 1s ease;
        min-width: 4px;
    }

    .bar-pct {
        width: 45px;
        text-align: right;
        font-weight: 700;
        font-size: 0.95rem;
        color: var(--color-text-secondary);
    }

    .total-votes {
        text-align: center;
        font-size: 0.9rem;
        color: var(--color-text-muted);
        margin-top: var(--spacing-sm);
        font-weight: 600;
    }
</style>
