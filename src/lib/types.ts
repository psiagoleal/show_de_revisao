// Caminho relativo: src/lib/types.ts
/// \file src/lib/types.ts
/// \brief TypeScript interfaces matching Rust backend structs
/// \author Iago Souza

/** Configuração de prêmios do jogo */
export interface PremioConfig {
    nome: string;
    icone: string;
    valores: number[];
}

/** Configuração das ajudas disponíveis */
export interface AjudasConfig {
    universitarios: boolean;
    cartas: boolean;
    pular: number;
}

/** Configuração do timer */
export interface TimerConfig {
    habilitado: boolean;
    segundos: number;
}

/** Configuração de efeitos sonoros */
export interface SonsConfig {
    inicio: string | null;
    pergunta: string | null;
    suspense: string | null;
    acerto: string | null;
    erro: string | null;
    timeout: string | null;
    ajuda: string | null;
    vitoria: string | null;
    parar: string | null;
}

/** Configuração geral do jogo */
export interface GameConfig {
    titulo: string;
    rodadas: number;
    premio: PremioConfig;
    ajudas: AjudasConfig;
    checkpoints: number[];
    timer: TimerConfig;
    sons: SonsConfig;
}

/** Alternativas de uma pergunta */
export interface Alternativas {
    A: string;
    B: string;
    C: string;
    D: string;
}

/** Uma pergunta do quiz */
export interface Pergunta {
    texto: string;
    alternativas: Alternativas;
    correta: string;
}

/** Status atual do jogo */
export type GameStatus =
    | "AguardandoInicio"
    | "MostrandoPergunta"
    | "RespostaSelecionada"
    | "RespostaRevelada"
    | "Finalizado";

/** Resultado final do jogo */
export type ResultadoFinal = "Vitoria" | "Derrota" | "Parou" | "Timeout";

/** Status das ajudas utilizadas */
export interface AjudasUsadas {
    universitarios: boolean;
    cartas: boolean;
    pulos_usados: number;
}

/** Resultado da ajuda dos universitários */
export interface UniversityVotes {
    a: number;
    b: number;
    c: number;
    d: number;
    total_votos: number | null;
}

/** Resultado da ajuda das cartas */
export interface CartasResult {
    alternativas_visiveis: string[];
}

/** Estado completo do jogo */
export interface GameState {
    config: GameConfig;
    perguntas: Pergunta[];
    pergunta_atual: number;
    premio_acumulado: number;
    premio_garantido: number;
    status: GameStatus;
    ajudas_usadas: AjudasUsadas;
    resposta_selecionada: string | null;
    resposta_correta: boolean | null;
    votacao_em_andamento: boolean;
    university_votes: UniversityVotes | null;
    cartas_result: CartasResult | null;
    resultado_final: ResultadoFinal | null;
    timer_segundos_restantes: number | null;
    timer_ativo: boolean;
    /** Total de perguntas no pool */
    total_pool: number;
    /** Quantas perguntas restam no pool (não usadas) */
    pool_restante: number;
    /** Se o pool foi resetado nesta partida */
    pool_resetado: boolean;
}
