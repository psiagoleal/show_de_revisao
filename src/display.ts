// Caminho relativo: src/display.ts
/// \file src/display.ts
/// \brief Entry point for the Display (projector) window
/// \author Iago Souza

console.log("[ShowDeQuestao] display.ts loaded");

try {
    const { mount } = await import("svelte");
    console.log("[ShowDeQuestao] Svelte imported successfully (display)");

    const { default: DisplayApp } = await import("./display/DisplayApp.svelte");
    console.log("[ShowDeQuestao] DisplayApp imported successfully");

    const target = document.getElementById("app");
    if (!target) {
        throw new Error("Target element #app not found in DOM");
    }

    console.log("[ShowDeQuestao] Mounting DisplayApp...");
    const app = mount(DisplayApp, { target });
    console.log("[ShowDeQuestao] DisplayApp mounted successfully");
} catch (err) {
    console.error("[ShowDeQuestao] Failed to mount DisplayApp:", err);
    const target = document.getElementById("app");
    if (target) {
        target.innerHTML = `
      <div style="padding: 2rem; color: #f44336; font-family: monospace; background: #1a1a2e;">
        <h1 style="color: #f4a020;">⚠️ Erro ao iniciar Show de Questão (Display)</h1>
        <pre style="white-space: pre-wrap; margin-top: 1rem; color: #ff6b6b;">${err instanceof Error ? err.message : String(err)}</pre>
        <pre style="white-space: pre-wrap; margin-top: 0.5rem; color: #888; font-size: 0.8rem;">${err instanceof Error && err.stack ? err.stack : ""}</pre>
        <p style="margin-top: 1rem; color: #b0c4de;">Verifique o console (F12) para mais detalhes.</p>
      </div>
    `;
    }
}
