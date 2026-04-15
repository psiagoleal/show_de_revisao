// Caminho relativo: src-tauri/src/lib.rs
/// \file src-tauri/src/lib.rs
/// \brief Main library - Tauri application setup
/// \author Iago Souza
pub mod commands;
pub mod config;
pub mod game;

use commands::AppState;
use std::sync::Mutex;

/// Configura e executa a aplicação Tauri
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            game: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::carregar_jogo,
            commands::iniciar_jogo,
            commands::selecionar_resposta,
            commands::revelar_resposta,
            commands::proxima_pergunta,
            commands::usar_universitarios,
            commands::submeter_votos,
            commands::usar_cartas,
            commands::usar_pular,
            commands::parar_jogo,
            commands::iniciar_timer,
            commands::pausar_timer,
            commands::tick_timer,
            commands::reiniciar_jogo,
            commands::obter_estado,
        ])
        .run(tauri::generate_context!())
        .expect("Erro ao executar a aplicação Tauri");
}
