// Caminho relativo: src/main.ts
/// \file src/main.ts
/// \brief Entry point for the Presenter (control) window
/// \author Iago Souza

console.log("[ShowDeQuestao] main.ts loaded");

try {
    const { mount } = await import("svelte");
    console.log("[ShowDeQuestao] Svelte imported successfully");

    const { default: PresenterApp } =
        await import("./presenter/PresenterApp.svelte");
    console.log("[ShowDeQuestao] PresenterApp imported successfully");

    const target = document.getElementById("app");
    if (!target) {
        throw new Error("Target element #app not found in DOM");
    }

    console.log("[ShowDeQuestao] Mounting PresenterApp...");
    const app = mount(PresenterApp, { target });
    console.log("[ShowDeQuestao] PresenterApp mounted successfully");
} catch (err) {
    console.error("[ShowDeQuestao] Failed to mount PresenterApp:", err);
    const target = document.getElementById("app");
    if (target) {
        target.innerHTML = `
      <div style="padding: 2rem; color: #f44336; font-family: monospace; background: #1a1a2e;">
        <h1 style="color: #f4a020;">⚠️ Erro ao iniciar Show de Questão</h1>
        <pre style="white-space: pre-wrap; margin-top: 1rem; color: #ff6b6b;">${err instanceof Error ? err.message : String(err)}</pre>
        <pre style="white-space: pre-wrap; margin-top: 0.5rem; color: #888; font-size: 0.8rem;">${err instanceof Error && err.stack ? err.stack : ""}</pre>
        <p style="margin-top: 1rem; color: #b0c4de;">Verifique o console (F12) para mais detalhes.</p>
      </div>
    `;
    }
}
