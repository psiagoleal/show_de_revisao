# 🎯 Show de Questão

![Status](https://img.shields.io/badge/status-em%20desenvolvimento-yellow)
![Licença](https://img.shields.io/badge/licença-MIT-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.x-blue)
![Svelte](https://img.shields.io/badge/Svelte-5-orange)
![Rust](https://img.shields.io/badge/Rust-Backend-red)

Inspirado por minha esposa, que é professora, desenvolvi este programa para que ela possa utilizar em dinâmicas em sala de aula.

**Show de Questão** é um aplicativo de quiz interativo inspirado no programa de TV "Show do Milhão" (SBT), desenvolvido para uso em **salas de aula** e **brincadeiras entre amigos**.

O professor (apresentador) controla o jogo por uma janela, enquanto os alunos veem as perguntas e alternativas em outra janela (projetada em telão ou TV).

## ✨ Funcionalidades

- 🎮 **Duas janelas**: Controle (apresentador) + Apresentação (projetor/alunos)
- 📋 **Perguntas personalizáveis** via arquivo JSON
- 🏆 **Prêmios configuráveis** (dinheiro, pontos, pirulitos... qualquer coisa!)
- 🎓 **3 Ajudas**: Universitários, Cartas e Pular
- ⏱️ **Timer opcional** (configurável ou acionado manualmente)
- 🔊 **Efeitos sonoros** personalizáveis
- 📊 **Escada de prêmios** visual com checkpoints
- 🛑 **Parar o jogo** e manter o prêmio acumulado

## 📦 Pré-requisitos

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.70
- [Tauri CLI](https://tauri.app/) 2.x
- Sistema operacional: Windows, Linux ou macOS

## 🚀 Instalação

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/show-de-questao.git
cd show-de-questao

# Instale as dependências do frontend
npm install

# Execute em modo de desenvolvimento
npm run tauri dev

# Compile para produção
npm run tauri build
```

## 📖 Como Usar

1. Edite o arquivo `perguntas.json` com suas perguntas
2. Configure o `config.json` com prêmios, ajudas e timer
3. (Opcional) Adicione arquivos de som na pasta `sons/`
4. Execute o programa
5. Use a janela de **Controle** para gerenciar o jogo
6. Projete a janela de **Apresentação** para os alunos

## 📂 Estrutura do Projeto

```
show_de_questao/
├── src-tauri/              # Backend Rust + Tauri
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── lib.rs          # Biblioteca principal
│   │   ├── commands.rs     # Comandos Tauri
│   │   ├── game.rs         # Lógica do jogo
│   │   └── config.rs       # Leitura de configuração
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                    # Frontend Svelte 5
│   ├── lib/
│   │   ├── types.ts        # Interfaces TypeScript
│   │   ├── api.ts          # Wrappers dos comandos Tauri
│   │   └── events.ts       # Listeners de eventos Tauri
│   ├── presenter/          # Tela do Apresentador
│   │   ├── PresenterApp.svelte
│   │   ├── PresenterControls.svelte
│   │   ├── PresenterQuestion.svelte
│   │   ├── HelpControls.svelte
│   │   └── TimerControl.svelte
│   ├── display/            # Tela de Apresentação
│   │   ├── DisplayApp.svelte
│   │   ├── QuestionDisplay.svelte
│   │   ├── AnswerOption.svelte
│   │   ├── PrizeLadder.svelte
│   │   ├── HelpStatus.svelte
│   │   ├── TimerDisplay.svelte
│   │   ├── UniversityHelp.svelte
│   │   └── GameOverScreen.svelte
│   ├── main.ts             # Entry point do apresentador
│   └── display.ts          # Entry point do display
├── config.json             # Configuração do jogo
├── perguntas.json          # Perguntas do quiz
└── package.json
```

## ⚙️ Configuração

### `config.json`

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `titulo` | string | Nome do jogo |
| `premio.nome` | string | Nome do prêmio (ex: "pontos", "pirulitos") |
| `premio.icone` | string | Emoji do prêmio |
| `premio.valores` | number[] | Valores crescentes para cada pergunta |
| `ajudas.universitarios` | boolean | Habilitar ajuda dos universitários |
| `ajudas.cartas` | boolean | Habilitar ajuda das cartas |
| `ajudas.pular` | number | Quantidade de pulos disponíveis |
| `checkpoints` | number[] | Índices das perguntas que são checkpoints |
| `timer.habilitado` | boolean | Timer automático habilitado |
| `timer.segundos` | number | Segundos por pergunta |

### `perguntas.json`

Cada pergunta deve ter: `texto`, `alternativas` (A, B, C, D) e `correta`.

## 📄 Licença

Este projeto está licenciado sob a [Licença MIT](LICENSE).

---

## Apoie

**Feito com ❤️ por Iago Leal** | [☕ Apoie o criador]

Se este projeto ajudou você, considere apoiar:

- Buy Me a Coffee: https://buymeacoffee.com/psiagoleal

<a href="https://buymeacoffee.com/psiagoleal" target="_blank" rel="noopener">
  <img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" height="41" width="174" />
</a>
