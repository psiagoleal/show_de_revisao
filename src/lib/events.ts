// Caminho relativo: src/lib/events.ts
/// \file src/lib/events.ts
/// \brief Tauri event listeners for the display window
/// \author Iago Souza

import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { GameState } from "./types";

/**
 * Escuta atualizações de estado do jogo emitidas pelo backend
 * Usado pela janela de display para receber atualizações em tempo real
 *
 * @param callback - Função chamada quando o estado é atualizado
 * @returns Função para cancelar a escuta
 */
export async function onEstadoAtualizado(
  callback: (state: GameState) => void,
): Promise<UnlistenFn> {
  return await listen<GameState>("estado-atualizado", (event) => {
    callback(event.payload);
  });
}
