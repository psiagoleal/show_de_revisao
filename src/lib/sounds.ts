// Caminho relativo: src/lib/sounds.ts
/// \file src/lib/sounds.ts
/// \brief Sound effects management
/// \author Iago Souza

import type { SonsConfig } from "./types";

/** Tipos de sons disponíveis */
export type SoundType = keyof SonsConfig;

/** Cache de objetos Audio para reprodução */
const audioCache: Map<string, HTMLAudioElement> = new Map();

/**
 * Reproduz um efeito sonoro se configurado
 *
 * @param sonsConfig - Configuração de sons do jogo
 * @param tipo - Tipo do som a reproduzir
 */
export function playSound(sonsConfig: SonsConfig, tipo: SoundType): void {
  const arquivo = sonsConfig[tipo];
  if (!arquivo) return;

  try {
    let audio = audioCache.get(arquivo);
    if (!audio) {
      audio = new Audio(arquivo);
      audioCache.set(arquivo, audio);
    }
    audio.currentTime = 0;
    audio.play().catch((err) => {
      console.warn(`Não foi possível reproduzir o som '${tipo}':`, err);
    });
  } catch (err) {
    console.warn(`Erro ao criar áudio para '${tipo}':`, err);
  }
}

/**
 * Para todos os sons em reprodução
 */
export function stopAllSounds(): void {
  audioCache.forEach((audio) => {
    audio.pause();
    audio.currentTime = 0;
  });
}
