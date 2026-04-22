// Caminho relativo: src-tauri/src/history.rs
/// \file src-tauri/src/history.rs
/// \brief Persistent question history — tracks used questions across sessions
/// \author Iago Souza
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Registro persistente de perguntas já utilizadas, indexado pelo caminho absoluto do arquivo
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoricoLog {
    /// Mapa: caminho absoluto do arquivo JSON → índices das perguntas já utilizadas
    pub historico: HashMap<String, Vec<usize>>,
}

/// Retorna o caminho do arquivo de log de histórico
pub fn historico_path(base_path: &Path) -> PathBuf {
    base_path.join("historico_perguntas.json")
}

/// Carrega o histórico do disco. Retorna vazio se não existir ou for inválido.
pub fn carregar_historico(base_path: &Path) -> HistoricoLog {
    let path = historico_path(base_path);
    if !path.exists() {
        return HistoricoLog::default();
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return HistoricoLog::default(),
    };
    serde_json::from_str(&content).unwrap_or_default()
}

/// Salva o histórico no disco
pub fn salvar_historico(base_path: &Path, historico: &HistoricoLog) -> Result<(), String> {
    let path = historico_path(base_path);
    let content = serde_json::to_string_pretty(historico)
        .map_err(|e| format!("Erro ao serializar histórico: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Erro ao salvar histórico em '{}': {}", path.display(), e))?;
    Ok(())
}

/// Obtém os índices já usados para um arquivo específico
pub fn obter_indices_usados(historico: &HistoricoLog, arquivo: &str) -> Vec<usize> {
    historico
        .historico
        .get(arquivo)
        .cloned()
        .unwrap_or_default()
}

/// Atualiza os índices usados para um arquivo específico
pub fn atualizar_indices(historico: &mut HistoricoLog, arquivo: &str, indices: &[usize]) {
    historico
        .historico
        .insert(arquivo.to_string(), indices.to_vec());
}

/// Reseta o histórico para um arquivo específico (permite reusar todas as perguntas)
pub fn resetar_arquivo(historico: &mut HistoricoLog, arquivo: &str) {
    historico.historico.remove(arquivo);
}

/// Informações de histórico para um arquivo de perguntas específico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoHistorico {
    /// Caminho absoluto do arquivo
    pub arquivo: String,
    /// Nome do arquivo (sem o path)
    pub nome_arquivo: String,
    /// Total de perguntas no arquivo
    pub total: usize,
    /// Quantas já foram utilizadas
    pub usados: usize,
    /// Quantas ainda estão disponíveis
    pub disponiveis: usize,
}
