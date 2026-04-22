// Caminho relativo: src/lib/api.ts
/// \file src/lib/api.ts
/// \brief Tauri command wrappers - API bridge to Rust backend
/// \author Iago Souza

import { invoke } from "@tauri-apps/api/core";
import type { GameState, InfoHistorico } from "./types";

/**
 * Carrega o jogo a partir dos arquivos config.json e do arquivo de perguntas escolhido.
 * @param perguntas_path - Caminho absoluto para o arquivo de perguntas JSON.
 *   Se vazio ou omitido, usa o arquivo padrão `perguntas.json` ao lado do executável.
 */
export async function carregarJogo(
    perguntas_path: string = "",
): Promise<GameState> {
    return await invoke<GameState>("carregar_jogo", {
        perguntasPath: perguntas_path,
    });
}

/**
 * Inicia uma nova partida
 */
export async function iniciarJogo(): Promise<GameState> {
    return await invoke<GameState>("iniciar_jogo");
}

/**
 * Seleciona uma resposta (A, B, C ou D)
 */
export async function selecionarResposta(opcao: string): Promise<GameState> {
    return await invoke<GameState>("selecionar_resposta", { opcao });
}

/**
 * Revela se a resposta selecionada está correta
 */
export async function revelarResposta(): Promise<GameState> {
    return await invoke<GameState>("revelar_resposta");
}

/**
 * Avança para a próxima pergunta
 */
export async function proximaPergunta(): Promise<GameState> {
    return await invoke<GameState>("proxima_pergunta");
}

/**
 * Usa a ajuda dos universitários
 */
export async function usarUniversitarios(): Promise<GameState> {
    return await invoke<GameState>("usar_universitarios");
}

/**
 * Usa a ajuda das cartas (elimina 2 alternativas erradas)
 */
export async function usarCartas(): Promise<GameState> {
    return await invoke<GameState>("usar_cartas");
}

/**
 * Usa a ajuda de pular (pula para a próxima pergunta)
 */
export async function usarPular(): Promise<GameState> {
    return await invoke<GameState>("usar_pular");
}

/**
 * Para o jogo e mantém o prêmio acumulado
 */
export async function pararJogo(): Promise<GameState> {
    return await invoke<GameState>("parar_jogo");
}

/**
 * Inicia o timer manualmente
 * @param segundos - Segundos opcionais (usa o padrão da config se não informado)
 */
export async function iniciarTimer(segundos?: number): Promise<GameState> {
    return await invoke<GameState>("iniciar_timer", {
        segundos: segundos ?? null,
    });
}

/**
 * Pausa o timer
 */
export async function pausarTimer(): Promise<GameState> {
    return await invoke<GameState>("pausar_timer");
}

/**
 * Tick do timer (chamado a cada segundo pelo frontend)
 */
export async function tickTimer(): Promise<GameState> {
    return await invoke<GameState>("tick_timer");
}

/**
 * Reinicia o jogo
 */
export async function reiniciarJogo(): Promise<GameState> {
    return await invoke<GameState>("reiniciar_jogo");
}

/**
 * Submete os votos reais da turma para a ajuda dos universitários
 */
export async function submeterVotos(
    a: number,
    b: number,
    c: number,
    d: number,
): Promise<GameState> {
    return await invoke<GameState>("submeter_votos", { a, b, c, d });
}

/**
 * Obtém o estado atual do jogo
 */
export async function obterEstado(): Promise<GameState | null> {
    return await invoke<GameState | null>("obter_estado");
}

/**
 * Obtém informações de histórico para um arquivo de perguntas específico.
 * @param arquivo - Caminho absoluto do arquivo de perguntas JSON.
 */
export async function obterInfoHistorico(
    arquivo: string,
): Promise<InfoHistorico> {
    return await invoke<InfoHistorico>("obter_info_historico", { arquivo });
}

/**
 * Reseta o histórico de perguntas usadas para um arquivo específico.
 * Após o reset, todas as perguntas do arquivo estarão disponíveis novamente.
 * @param arquivo - Caminho absoluto do arquivo de perguntas JSON.
 */
export async function resetarHistorico(arquivo: string): Promise<void> {
    return await invoke<void>("resetar_historico", { arquivo });
}
