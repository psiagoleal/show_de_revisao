// Caminho relativo: src-tauri/src/commands.rs
/// \file src-tauri/src/commands.rs
/// \brief Tauri commands - API bridge between Rust backend and Svelte frontend
/// \author Iago Souza
use crate::config;
use crate::game::GameState;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

/// Estado global do jogo gerenciado pelo Tauri
pub struct AppState {
    pub game: Mutex<Option<GameState>>,
}

/// Resolve o caminho base para os arquivos de configuração.
/// Em modo de desenvolvimento, usa o diretório do projeto.
/// Em produção, usa o diretório do executável.
fn resolve_base_path(_app: &AppHandle) -> PathBuf {
    // Em produção, usar o diretório do executável
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let config_path = parent.join("config.json");
            if config_path.exists() {
                return parent.to_path_buf();
            }
        }
    }

    // Fallback: diretório de trabalho atual
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Carrega o jogo a partir dos arquivos de configuração
#[tauri::command]
pub fn carregar_jogo(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let base_path = resolve_base_path(&app);

    let config_path = base_path.join("config.json");
    let perguntas_path = base_path.join("perguntas.json");

    let config = config::load_config(&config_path)?;
    let perguntas = config::load_perguntas(&perguntas_path)?;

    // Validar que há perguntas suficientes para o número de rodadas
    if perguntas.len() < config.rodadas {
        return Err(format!(
            "O arquivo de perguntas tem {} perguntas, mas são necessárias pelo menos {} (número de rodadas)",
            perguntas.len(),
            config.rodadas
        ));
    }

    let game_state = GameState::new(config, perguntas);

    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    *game = Some(game_state.clone());

    // Emitir evento para a janela de display
    let _ = app.emit_to("display", "estado-atualizado", &game_state);

    Ok(game_state)
}

/// Inicia uma nova partida
#[tauri::command]
pub fn iniciar_jogo(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game
        .as_mut()
        .ok_or("Jogo não carregado. Carregue o jogo primeiro.")?;

    game_state.iniciar()?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Seleciona uma resposta
#[tauri::command]
pub fn selecionar_resposta(
    app: AppHandle,
    state: State<AppState>,
    opcao: String,
) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.selecionar_resposta(opcao)?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Revela a resposta correta
#[tauri::command]
pub fn revelar_resposta(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.revelar_resposta()?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Avança para a próxima pergunta
#[tauri::command]
pub fn proxima_pergunta(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.proxima_pergunta()?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Usa a ajuda dos universitários (inicia votação em sala)
#[tauri::command]
pub fn usar_universitarios(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.usar_universitarios()?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Submete os votos reais da sala para a ajuda dos universitários
#[tauri::command]
pub fn submeter_votos(
    app: AppHandle,
    state: State<AppState>,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.submeter_votos(a, b, c, d)?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Usa a ajuda das cartas
#[tauri::command]
pub fn usar_cartas(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.usar_cartas()?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Usa a ajuda de pular
#[tauri::command]
pub fn usar_pular(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.usar_pular()?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Para o jogo e mantém o prêmio
#[tauri::command]
pub fn parar_jogo(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.parar()?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Inicia o timer manualmente
#[tauri::command]
pub fn iniciar_timer(
    app: AppHandle,
    state: State<AppState>,
    segundos: Option<u32>,
) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.iniciar_timer(segundos)?;

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Pausa o timer
#[tauri::command]
pub fn pausar_timer(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.pausar_timer();

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Tick do timer (chamado pelo frontend a cada segundo)
#[tauri::command]
pub fn tick_timer(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    if let Some(remaining) = game_state.tick_timer() {
        if remaining == 0 && game_state.timer_ativo {
            game_state.timeout()?;
        }
    }

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Reinicia o jogo
#[tauri::command]
pub fn reiniciar_jogo(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;

    game_state.reiniciar();

    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());

    Ok(game_state.clone())
}

/// Obtém o estado atual do jogo
#[tauri::command]
pub fn obter_estado(state: State<AppState>) -> Result<Option<GameState>, String> {
    let game = state.game.lock().map_err(|e| e.to_string())?;
    Ok(game.clone())
}
