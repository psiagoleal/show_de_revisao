// Caminho relativo: src-tauri/src/config.rs
/// \file src-tauri/src/config.rs
/// \brief Configuration loading and validation from JSON files
/// \author Iago Souza
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Configuração de prêmios do jogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremioConfig {
    /// Nome do prêmio (ex: "pontos", "pirulitos", "reais")
    pub nome: String,
    /// Emoji representando o prêmio
    pub icone: String,
    /// Valores crescentes para cada pergunta
    pub valores: Vec<u64>,
}

/// Configuração das ajudas disponíveis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AjudasConfig {
    /// Se a ajuda dos universitários está habilitada
    pub universitarios: bool,
    /// Se a ajuda das cartas está habilitada
    pub cartas: bool,
    /// Quantidade de pulos disponíveis
    pub pular: u32,
}

/// Configuração do timer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerConfig {
    /// Se o timer automático está habilitado
    pub habilitado: bool,
    /// Segundos por pergunta
    pub segundos: u32,
}

/// Configuração de efeitos sonoros
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SonsConfig {
    /// Som de início do jogo
    pub inicio: Option<String>,
    /// Som ao exibir uma pergunta
    pub pergunta: Option<String>,
    /// Som de suspense antes de revelar resposta
    pub suspense: Option<String>,
    /// Som de acerto
    pub acerto: Option<String>,
    /// Som de erro
    pub erro: Option<String>,
    /// Som de timeout
    pub timeout: Option<String>,
    /// Som ao usar ajuda
    pub ajuda: Option<String>,
    /// Som de vitória (ganhou o prêmio máximo)
    pub vitoria: Option<String>,
    /// Som ao parar o jogo
    pub parar: Option<String>,
}

/// Configuração geral do jogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Título do jogo
    pub titulo: String,
    /// Número de rodadas (perguntas por partida)
    pub rodadas: usize,
    /// Configuração de prêmios
    pub premio: PremioConfig,
    /// Configuração das ajudas
    pub ajudas: AjudasConfig,
    /// Índices das perguntas que são checkpoints (prêmio garantido)
    pub checkpoints: Vec<usize>,
    /// Configuração do timer
    pub timer: TimerConfig,
    /// Configuração de efeitos sonoros
    pub sons: SonsConfig,
}

/// Alternativas de uma pergunta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternativas {
    #[serde(rename = "A")]
    pub a: String,
    #[serde(rename = "B")]
    pub b: String,
    #[serde(rename = "C")]
    pub c: String,
    #[serde(rename = "D")]
    pub d: String,
}

/// Uma pergunta do quiz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pergunta {
    /// Texto da pergunta
    pub texto: String,
    /// Alternativas A, B, C e D
    pub alternativas: Alternativas,
    /// Letra da resposta correta ("A", "B", "C" ou "D")
    pub correta: String,
}

/// Arquivo de perguntas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerguntasFile {
    /// Lista de perguntas
    pub perguntas: Vec<Pergunta>,
}

/// Carrega a configuração do jogo a partir de um arquivo JSON
///
/// # Arguments
/// * `path` - Caminho para o arquivo config.json
///
/// # Returns
/// * `Result<GameConfig, String>` - Configuração carregada ou erro
pub fn load_config(path: &Path) -> Result<GameConfig, String> {
    let content = fs::read_to_string(path).map_err(|e| {
        format!(
            "Erro ao ler arquivo de configuração '{}': {}",
            path.display(),
            e
        )
    })?;

    let config: GameConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Erro ao interpretar configuração JSON: {}", e))?;

    validate_config(&config)?;

    Ok(config)
}

/// Carrega as perguntas a partir de um arquivo JSON
///
/// # Arguments
/// * `path` - Caminho para o arquivo perguntas.json
///
/// # Returns
/// * `Result<Vec<Pergunta>, String>` - Lista de perguntas ou erro
pub fn load_perguntas(path: &Path) -> Result<Vec<Pergunta>, String> {
    let content = fs::read_to_string(path).map_err(|e| {
        format!(
            "Erro ao ler arquivo de perguntas '{}': {}",
            path.display(),
            e
        )
    })?;

    let file: PerguntasFile = serde_json::from_str(&content)
        .map_err(|e| format!("Erro ao interpretar perguntas JSON: {}", e))?;

    validate_perguntas(&file.perguntas)?;

    Ok(file.perguntas)
}

/// Valida a configuração do jogo
fn validate_config(config: &GameConfig) -> Result<(), String> {
    if config.premio.valores.is_empty() {
        return Err("A lista de valores de prêmio não pode estar vazia".to_string());
    }

    if config.premio.nome.is_empty() {
        return Err("O nome do prêmio não pode estar vazio".to_string());
    }

    // Verificar se os valores estão em ordem crescente
    for i in 1..config.premio.valores.len() {
        if config.premio.valores[i] <= config.premio.valores[i - 1] {
            return Err(format!(
                "Os valores de prêmio devem estar em ordem crescente. Valor {} ({}) não é maior que {} ({})",
                i, config.premio.valores[i], i - 1, config.premio.valores[i - 1]
            ));
        }
    }

    // Verificar checkpoints válidos
    for &cp in &config.checkpoints {
        if cp >= config.premio.valores.len() {
            return Err(format!(
                "Checkpoint {} está fora do intervalo (máximo: {})",
                cp,
                config.premio.valores.len() - 1
            ));
        }
    }

    if config.rodadas == 0 {
        return Err("O número de rodadas deve ser maior que 0".to_string());
    }

    if config.rodadas != config.premio.valores.len() {
        return Err(format!(
            "O número de rodadas ({}) deve ser igual ao número de valores de prêmio ({})",
            config.rodadas,
            config.premio.valores.len()
        ));
    }

    if config.timer.habilitado && config.timer.segundos == 0 {
        return Err("O timer está habilitado mas os segundos são 0".to_string());
    }

    Ok(())
}

/// Valida a lista de perguntas
fn validate_perguntas(perguntas: &[Pergunta]) -> Result<(), String> {
    if perguntas.is_empty() {
        return Err("A lista de perguntas não pode estar vazia".to_string());
    }

    for (i, p) in perguntas.iter().enumerate() {
        if p.texto.is_empty() {
            return Err(format!("Pergunta {} tem texto vazio", i + 1));
        }

        let correta = p.correta.to_uppercase();
        if !["A", "B", "C", "D"].contains(&correta.as_str()) {
            return Err(format!(
                "Pergunta {} tem resposta correta inválida: '{}'. Deve ser A, B, C ou D",
                i + 1,
                p.correta
            ));
        }
    }

    Ok(())
}
