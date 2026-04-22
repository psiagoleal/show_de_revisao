// Caminho relativo: src-tauri/src/game.rs
/// \file src-tauri/src/game.rs
/// \brief Game state management and logic
/// \author Iago Souza
use crate::config::{GameConfig, Pergunta};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

/// Status atual do jogo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
    /// Aguardando início do jogo
    AguardandoInicio,
    /// Mostrando uma pergunta, aguardando resposta
    MostrandoPergunta,
    /// Participante selecionou uma resposta, aguardando revelação
    RespostaSelecionada,
    /// Resposta revelada (correta ou errada)
    RespostaRevelada,
    /// Jogo finalizado (ganhou, perdeu ou parou)
    Finalizado,
}

/// Resultado final do jogo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResultadoFinal {
    /// Ganhou o prêmio máximo
    Vitoria,
    /// Errou e voltou ao checkpoint
    Derrota,
    /// Parou por vontade própria
    Parou,
    /// Tempo esgotado
    Timeout,
}

/// Status das ajudas utilizadas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AjudasUsadas {
    /// Se a ajuda dos universitários já foi usada
    pub universitarios: bool,
    /// Se a ajuda das cartas já foi usada
    pub cartas: bool,
    /// Quantos pulos já foram usados
    pub pulos_usados: u32,
}

/// Resultado da ajuda dos universitários
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversityVotes {
    /// Porcentagem para alternativa A
    pub a: u32,
    /// Porcentagem para alternativa B
    pub b: u32,
    /// Porcentagem para alternativa C
    pub c: u32,
    /// Porcentagem para alternativa D
    pub d: u32,
    /// Total de votos recebidos (presente apenas com votos reais)
    pub total_votos: Option<u32>,
}

/// Resultado da ajuda das cartas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartasResult {
    /// Alternativas que permanecem visíveis (a correta + 1 errada)
    pub alternativas_visiveis: Vec<String>,
}

/// Estado completo do jogo (enviado ao frontend)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Configuração do jogo
    pub config: GameConfig,
    /// Lista de perguntas selecionadas para a partida atual
    pub perguntas: Vec<Pergunta>,
    /// Pool completo de todas as perguntas disponíveis
    #[serde(skip_serializing)]
    pub todas_perguntas: Vec<Pergunta>,
    /// Caminho absoluto do arquivo de perguntas (chave para o histórico)
    pub arquivo_perguntas: String,
    /// Índices das perguntas já utilizadas nesta sessão (persiste entre partidas)
    pub indices_usados: Vec<usize>,
    /// Total de perguntas no pool
    pub total_pool: usize,
    /// Quantas perguntas restam no pool (não usadas)
    pub pool_restante: usize,
    /// Se o pool foi resetado nesta partida
    pub pool_resetado: bool,
    /// Índice da pergunta atual (0-based)
    pub pergunta_atual: usize,
    /// Prêmio acumulado atual
    pub premio_acumulado: u64,
    /// Prêmio garantido (último checkpoint alcançado)
    pub premio_garantido: u64,
    /// Status do jogo
    pub status: GameStatus,
    /// Ajudas utilizadas
    pub ajudas_usadas: AjudasUsadas,
    /// Resposta selecionada pelo participante
    pub resposta_selecionada: Option<String>,
    /// Se a resposta foi correta (após revelação)
    pub resposta_correta: Option<bool>,
    /// Resultado da ajuda dos universitários (se ativa)
    pub university_votes: Option<UniversityVotes>,
    /// Resultado da ajuda das cartas (se ativa)
    pub cartas_result: Option<CartasResult>,
    /// Resultado final do jogo
    pub resultado_final: Option<ResultadoFinal>,
    /// Segundos restantes do timer
    pub timer_segundos_restantes: Option<u32>,
    /// Se o timer está rodando
    pub timer_ativo: bool,
    /// Se há uma votação dos universitários em andamento
    pub votacao_em_andamento: bool,
}

impl GameState {
    /// Cria um novo estado de jogo
    pub fn new(
        config: GameConfig,
        todas_perguntas: Vec<Pergunta>,
        arquivo_perguntas: String,
        indices_usados_iniciais: Vec<usize>,
    ) -> Self {
        let total_pool = todas_perguntas.len();
        let usados_count = indices_usados_iniciais.len().min(total_pool);
        let pool_restante = total_pool.saturating_sub(usados_count);
        Self {
            config,
            perguntas: Vec::new(),
            todas_perguntas,
            arquivo_perguntas,
            indices_usados: indices_usados_iniciais,
            total_pool,
            pool_restante,
            pool_resetado: false,
            pergunta_atual: 0,
            premio_acumulado: 0,
            premio_garantido: 0,
            status: GameStatus::AguardandoInicio,
            ajudas_usadas: AjudasUsadas {
                universitarios: false,
                cartas: false,
                pulos_usados: 0,
            },
            resposta_selecionada: None,
            resposta_correta: None,
            university_votes: None,
            cartas_result: None,
            resultado_final: None,
            timer_segundos_restantes: None,
            timer_ativo: false,
            votacao_em_andamento: false,
        }
    }

    /// Inicia o jogo
    pub fn iniciar(&mut self) -> Result<(), String> {
        if self.todas_perguntas.is_empty() {
            return Err("Não há perguntas carregadas".to_string());
        }

        let rodadas = self.config.rodadas;

        // Calcular índices disponíveis (não usados)
        let mut available: Vec<usize> = (0..self.todas_perguntas.len())
            .filter(|i| !self.indices_usados.contains(i))
            .collect();

        // Se não há perguntas suficientes, resetar o pool
        if available.len() < rodadas {
            self.indices_usados.clear();
            available = (0..self.todas_perguntas.len()).collect();
            self.pool_resetado = true;
        } else {
            self.pool_resetado = false;
        }

        // Selecionar aleatoriamente `rodadas` perguntas
        let mut rng = thread_rng();
        available.shuffle(&mut rng);
        let selected: Vec<usize> = available.into_iter().take(rodadas).collect();

        // Registrar índices usados
        self.indices_usados.extend(&selected);

        // Montar lista de perguntas da partida
        self.perguntas = selected
            .iter()
            .map(|&i| self.todas_perguntas[i].clone())
            .collect();

        // Atualizar contagem do pool
        self.pool_restante = self.todas_perguntas.len() - self.indices_usados.len();

        self.pergunta_atual = 0;
        self.premio_acumulado = 0;
        self.premio_garantido = 0;
        self.status = GameStatus::MostrandoPergunta;
        self.ajudas_usadas = AjudasUsadas {
            universitarios: false,
            cartas: false,
            pulos_usados: 0,
        };
        self.resposta_selecionada = None;
        self.resposta_correta = None;
        self.university_votes = None;
        self.cartas_result = None;
        self.resultado_final = None;
        self.timer_segundos_restantes = if self.config.timer.habilitado {
            Some(self.config.timer.segundos)
        } else {
            None
        };
        self.timer_ativo = false;
        self.votacao_em_andamento = false;

        Ok(())
    }

    /// Seleciona uma resposta
    pub fn selecionar_resposta(&mut self, opcao: String) -> Result<(), String> {
        if self.status != GameStatus::MostrandoPergunta {
            return Err("Não é possível selecionar resposta neste momento".to_string());
        }

        let opcao_upper = opcao.to_uppercase();
        if !["A", "B", "C", "D"].contains(&opcao_upper.as_str()) {
            return Err(format!("Opção inválida: {}", opcao));
        }

        // Verificar se a opção não foi eliminada pelas cartas
        if let Some(ref cartas) = self.cartas_result {
            if !cartas.alternativas_visiveis.contains(&opcao_upper) {
                return Err("Esta alternativa foi eliminada pela ajuda das cartas".to_string());
            }
        }

        self.resposta_selecionada = Some(opcao_upper);
        self.status = GameStatus::RespostaSelecionada;
        self.timer_ativo = false;

        Ok(())
    }

    /// Revela se a resposta está correta
    pub fn revelar_resposta(&mut self) -> Result<bool, String> {
        if self.status != GameStatus::RespostaSelecionada {
            return Err("Nenhuma resposta foi selecionada".to_string());
        }

        let pergunta = &self.perguntas[self.pergunta_atual];
        let correta = pergunta.correta.to_uppercase();
        let selecionada = self.resposta_selecionada.as_ref().unwrap();
        let acertou = selecionada == &correta;

        self.resposta_correta = Some(acertou);
        self.status = GameStatus::RespostaRevelada;

        if acertou {
            // Atualizar prêmio acumulado
            self.premio_acumulado = self.config.premio.valores[self.pergunta_atual];

            // Atualizar prêmio garantido se é checkpoint
            if self.config.checkpoints.contains(&self.pergunta_atual) {
                self.premio_garantido = self.premio_acumulado;
            }

            // Verificar se ganhou (última pergunta)
            if self.pergunta_atual >= self.config.premio.valores.len() - 1 {
                self.status = GameStatus::Finalizado;
                self.resultado_final = Some(ResultadoFinal::Vitoria);
            }
        } else {
            // Errou - prêmio volta ao checkpoint
            self.premio_acumulado = self.premio_garantido;
            self.status = GameStatus::Finalizado;
            self.resultado_final = Some(ResultadoFinal::Derrota);
        }

        Ok(acertou)
    }

    /// Avança para a próxima pergunta
    pub fn proxima_pergunta(&mut self) -> Result<(), String> {
        if self.status != GameStatus::RespostaRevelada {
            return Err("A resposta ainda não foi revelada".to_string());
        }

        if self.resultado_final.is_some() {
            return Err("O jogo já foi finalizado".to_string());
        }

        self.pergunta_atual += 1;
        self.status = GameStatus::MostrandoPergunta;
        self.resposta_selecionada = None;
        self.resposta_correta = None;
        self.university_votes = None;
        self.cartas_result = None;
        self.timer_segundos_restantes = if self.config.timer.habilitado {
            Some(self.config.timer.segundos)
        } else {
            None
        };
        self.timer_ativo = false;
        self.votacao_em_andamento = false;

        Ok(())
    }

    /// Usa a ajuda dos universitários (inicia fluxo de votação real)
    pub fn usar_universitarios(&mut self) -> Result<(), String> {
        if self.status != GameStatus::MostrandoPergunta {
            return Err("Só é possível usar ajuda durante uma pergunta".to_string());
        }

        if !self.config.ajudas.universitarios {
            return Err("A ajuda dos universitários não está habilitada".to_string());
        }

        if self.ajudas_usadas.universitarios {
            return Err("A ajuda dos universitários já foi utilizada".to_string());
        }

        self.ajudas_usadas.universitarios = true;
        self.votacao_em_andamento = true;

        Ok(())
    }

    /// Submete os votos reais da turma para a ajuda dos universitários
    pub fn submeter_votos(
        &mut self,
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    ) -> Result<UniversityVotes, String> {
        if !self.votacao_em_andamento {
            return Err("Não há votação em andamento".to_string());
        }

        if self.status != GameStatus::MostrandoPergunta {
            return Err("Só é possível submeter votos durante uma pergunta".to_string());
        }

        let total = a + b + c + d;
        if total == 0 {
            return Err("Nenhum voto foi registrado".to_string());
        }

        // Calcular porcentagens com divisão inteira
        let mut pct_a = (a * 100) / total;
        let mut pct_b = (b * 100) / total;
        let mut pct_c = (c * 100) / total;
        let mut pct_d = (d * 100) / total;

        // Ajustar para que a soma seja exatamente 100
        let soma = pct_a + pct_b + pct_c + pct_d;
        let remainder = 100 - soma;

        if remainder > 0 {
            // Adicionar o resto à maior porcentagem
            let max_val = pct_a.max(pct_b).max(pct_c).max(pct_d);
            if pct_a == max_val {
                pct_a += remainder;
            } else if pct_b == max_val {
                pct_b += remainder;
            } else if pct_c == max_val {
                pct_c += remainder;
            } else {
                pct_d += remainder;
            }
        }

        let votes = UniversityVotes {
            a: pct_a,
            b: pct_b,
            c: pct_c,
            d: pct_d,
            total_votos: Some(total),
        };

        self.university_votes = Some(votes.clone());
        self.votacao_em_andamento = false;

        Ok(votes)
    }

    /// Usa a ajuda das cartas (elimina 2 alternativas erradas)
    #[allow(unused_imports)]
    pub fn usar_cartas(&mut self) -> Result<CartasResult, String> {
        use rand::Rng;
        if self.status != GameStatus::MostrandoPergunta {
            return Err("Só é possível usar ajuda durante uma pergunta".to_string());
        }

        if !self.config.ajudas.cartas {
            return Err("A ajuda das cartas não está habilitada".to_string());
        }

        if self.ajudas_usadas.cartas {
            return Err("A ajuda das cartas já foi utilizada".to_string());
        }

        let pergunta = &self.perguntas[self.pergunta_atual];
        let correta = pergunta.correta.to_uppercase();

        // Pegar todas as alternativas erradas
        let mut erradas: Vec<String> = ["A", "B", "C", "D"]
            .iter()
            .filter(|&&x| x != correta.as_str())
            .map(|x| x.to_string())
            .collect();

        // Escolher aleatoriamente 1 errada para manter
        let mut rng = rand::thread_rng();
        let manter_idx = rng.gen_range(0..erradas.len());
        let errada_mantida = erradas.remove(manter_idx);

        let result = CartasResult {
            alternativas_visiveis: vec![correta, errada_mantida],
        };

        self.ajudas_usadas.cartas = true;
        self.cartas_result = Some(result.clone());

        Ok(result)
    }

    /// Usa a ajuda de pular
    pub fn usar_pular(&mut self) -> Result<(), String> {
        if self.status != GameStatus::MostrandoPergunta {
            return Err("Só é possível pular durante uma pergunta".to_string());
        }

        if self.ajudas_usadas.pulos_usados >= self.config.ajudas.pular {
            return Err(format!(
                "Todos os {} pulos já foram utilizados",
                self.config.ajudas.pular
            ));
        }

        self.ajudas_usadas.pulos_usados += 1;

        // Avançar para a próxima pergunta sem ganhar prêmio
        self.pergunta_atual += 1;
        self.resposta_selecionada = None;
        self.resposta_correta = None;
        self.university_votes = None;
        self.cartas_result = None;
        self.timer_segundos_restantes = if self.config.timer.habilitado {
            Some(self.config.timer.segundos)
        } else {
            None
        };
        self.timer_ativo = false;

        // Verificar se ainda há perguntas
        if self.pergunta_atual >= self.perguntas.len() {
            self.status = GameStatus::Finalizado;
            self.resultado_final = Some(ResultadoFinal::Parou);
        }

        Ok(())
    }

    /// Para o jogo e mantém o prêmio acumulado
    pub fn parar(&mut self) -> Result<u64, String> {
        if self.status == GameStatus::Finalizado {
            return Err("O jogo já foi finalizado".to_string());
        }

        let premio = self.premio_acumulado;
        self.status = GameStatus::Finalizado;
        self.resultado_final = Some(ResultadoFinal::Parou);
        self.timer_ativo = false;

        Ok(premio)
    }

    /// Lida com timeout do timer
    pub fn timeout(&mut self) -> Result<(), String> {
        self.premio_acumulado = self.premio_garantido;
        self.status = GameStatus::Finalizado;
        self.resultado_final = Some(ResultadoFinal::Timeout);
        self.timer_ativo = false;

        Ok(())
    }

    /// Inicia o timer manualmente
    pub fn iniciar_timer(&mut self, segundos: Option<u32>) -> Result<(), String> {
        if self.status != GameStatus::MostrandoPergunta {
            return Err("Só é possível iniciar o timer durante uma pergunta".to_string());
        }

        let secs = segundos.unwrap_or(self.config.timer.segundos);
        self.timer_segundos_restantes = Some(secs);
        self.timer_ativo = true;

        Ok(())
    }

    /// Pausa o timer
    pub fn pausar_timer(&mut self) {
        self.timer_ativo = false;
    }

    /// Decrementa o timer (chamado a cada segundo)
    pub fn tick_timer(&mut self) -> Option<u32> {
        if !self.timer_ativo {
            return self.timer_segundos_restantes;
        }

        if let Some(ref mut secs) = self.timer_segundos_restantes {
            if *secs > 0 {
                *secs -= 1;
            }
            Some(*secs)
        } else {
            None
        }
    }

    /// Reinicia o jogo (preserva pool e índices usados entre partidas)
    pub fn reiniciar(&mut self) {
        let config = self.config.clone();
        let todas_perguntas = self.todas_perguntas.clone();
        let indices_usados = self.indices_usados.clone();
        let arquivo_perguntas = self.arquivo_perguntas.clone();

        *self = GameState::new(config, todas_perguntas, arquivo_perguntas, indices_usados);
    }

    /// Retorna a pergunta atual
    pub fn pergunta_atual_ref(&self) -> Option<&Pergunta> {
        self.perguntas.get(self.pergunta_atual)
    }

    /// Retorna o valor do prêmio da pergunta atual
    pub fn premio_pergunta_atual(&self) -> Option<u64> {
        self.config.premio.valores.get(self.pergunta_atual).copied()
    }
}
