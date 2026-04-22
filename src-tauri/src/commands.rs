// Caminho relativo: src-tauri/src/commands.rs
/// \file src-tauri/src/commands.rs
/// \brief Tauri commands — API bridge between Rust backend and Svelte frontend
/// \author Iago Souza
use crate::config;
use crate::game::GameState;
use crate::history::{self, InfoHistorico};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

/// Estado global do jogo gerenciado pelo Tauri
pub struct AppState {
    pub game: Mutex<Option<GameState>>,
    /// Diretório base (onde o executável está) — usado para ler/salvar o histórico
    pub base_path: Mutex<Option<PathBuf>>,
}

/// Resolve o caminho base para os arquivos de configuração.
/// Em modo de desenvolvimento, usa o diretório do projeto.
/// Em produção, usa o diretório do executável.
fn resolve_base_path(_app: &AppHandle) -> PathBuf {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let config_path = parent.join("config.json");
            if config_path.exists() {
                return parent.to_path_buf();
            }
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Carrega o jogo a partir do arquivo de configuração e do arquivo de perguntas escolhido.
///
/// # Arguments
/// * `perguntas_path` — Caminho absoluto para o arquivo de perguntas JSON.
///   Se vazio, usa o arquivo padrão `perguntas.json` ao lado do executável.
#[tauri::command]
pub fn carregar_jogo(
    app: AppHandle,
    state: State<AppState>,
    perguntas_path: String,
) -> Result<GameState, String> {
    let base_path = resolve_base_path(&app);
    let config_path = base_path.join("config.json");

    // Determina qual arquivo de perguntas usar
    let perguntas_arquivo = if perguntas_path.is_empty() {
        base_path.join("perguntas.json")
    } else {
        PathBuf::from(&perguntas_path)
    };

    // Canonicalizar para obter o caminho absoluto real (chave do histórico)
    let perguntas_arquivo_canonical = perguntas_arquivo
        .canonicalize()
        .unwrap_or_else(|_| perguntas_arquivo.clone());
    let arquivo_key = perguntas_arquivo_canonical.to_string_lossy().to_string();

    let config = config::load_config(&config_path)?;
    let perguntas = config::load_perguntas(&perguntas_arquivo_canonical)?;

    if perguntas.len() < config.rodadas {
        return Err(format!(
            "O arquivo de perguntas tem {} perguntas, mas são necessárias pelo menos {} (número de rodadas configurado)",
            perguntas.len(),
            config.rodadas
        ));
    }

    // Carrega o histórico do disco para inicializar indices_usados
    let historico = history::carregar_historico(&base_path);
    let indices_usados = history::obter_indices_usados(&historico, &arquivo_key);

    let game_state = GameState::new(config, perguntas, arquivo_key, indices_usados);

    // Armazena o base_path para uso posterior (salvar histórico)
    {
        let mut bp = state.base_path.lock().map_err(|e| e.to_string())?;
        *bp = Some(base_path);
    }

    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    *game = Some(game_state.clone());

    let _ = app.emit_to("display", "estado-atualizado", &game_state);

    Ok(game_state)
}

/// Inicia uma nova partida e persiste o histórico de perguntas usadas
#[tauri::command]
pub fn iniciar_jogo(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game
        .as_mut()
        .ok_or("Jogo não carregado. Carregue o jogo primeiro.")?;

    game_state.iniciar()?;

    // Persiste os índices usados atualizados no disco
    let base_path = state.base_path.lock().map_err(|e| e.to_string())?;
    if let Some(bp) = base_path.as_ref() {
        let mut hist = history::carregar_historico(bp);
        history::atualizar_indices(
            &mut hist,
            &game_state.arquivo_perguntas,
            &game_state.indices_usados,
        );
        let _ = history::salvar_historico(bp, &hist);
    }

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

/// Usa a ajuda dos universitários
#[tauri::command]
pub fn usar_universitarios(app: AppHandle, state: State<AppState>) -> Result<GameState, String> {
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    let game_state = game.as_mut().ok_or("Jogo não carregado")?;
    game_state.usar_universitarios()?;
    let _ = app.emit_to("display", "estado-atualizado", &game_state.clone());
    Ok(game_state.clone())
}

/// Submete os votos reais da turma para a ajuda dos universitários
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

/// Reinicia o jogo (volta para AguardandoInicio preservando histórico em memória)
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

/// Obtém informações de histórico para um arquivo de perguntas específico.
/// Carrega o arquivo para saber o total de perguntas disponíveis.
#[tauri::command]
pub fn obter_info_historico(
    state: State<AppState>,
    arquivo: String,
) -> Result<InfoHistorico, String> {
    let base_path = state.base_path.lock().map_err(|e| e.to_string())?;
    let bp = base_path
        .as_ref()
        .ok_or("Base path não definido. Carregue o jogo primeiro.")?;

    let historico = history::carregar_historico(bp);
    let indices_usados = history::obter_indices_usados(&historico, &arquivo);

    // Carrega o arquivo para obter o total de perguntas
    let perguntas = config::load_perguntas(std::path::Path::new(&arquivo))?;
    let total = perguntas.len();
    let usados = indices_usados.len().min(total);
    let disponiveis = total.saturating_sub(usados);

    let nome_arquivo = std::path::Path::new(&arquivo)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| arquivo.clone());

    Ok(InfoHistorico {
        arquivo,
        nome_arquivo,
        total,
        usados,
        disponiveis,
    })
}

/// Reseta o histórico de perguntas usadas para um arquivo específico.
/// Após o reset, todas as perguntas do arquivo estarão disponíveis novamente.
#[tauri::command]
pub fn resetar_historico(state: State<AppState>, arquivo: String) -> Result<(), String> {
    let base_path = state.base_path.lock().map_err(|e| e.to_string())?;
    let bp = base_path
        .as_ref()
        .ok_or("Base path não definido. Carregue o jogo primeiro.")?;

    let mut historico = history::carregar_historico(bp);
    history::resetar_arquivo(&mut historico, &arquivo);
    history::salvar_historico(bp, &historico)?;

    // Atualiza o estado em memória se o arquivo resetado é o atual
    drop(base_path);
    let mut game = state.game.lock().map_err(|e| e.to_string())?;
    if let Some(gs) = game.as_mut() {
        if gs.arquivo_perguntas == arquivo {
            gs.indices_usados.clear();
            gs.pool_restante = gs.total_pool;
        }
    }

    Ok(())
}
