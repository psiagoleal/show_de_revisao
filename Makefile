# Caminho relativo: Makefile
# ============================================================================
# Show de Revisão — Makefile
# ============================================================================
# Comandos utilitários para desenvolvimento, build e distribuição do projeto.
#
# Uso:
#   make help          — Exibe todos os comandos disponíveis
#   make dev           — Inicia o ambiente de desenvolvimento
#   make build         — Gera o build de release para o SO atual
#
# Pré-requisitos:
#   - Node.js >= 18
#   - pnpm (gerenciador de pacotes)
#   - Rust toolchain (rustup, cargo)
#   - Dependências do Tauri para o SO alvo
#     (veja: https://v2.tauri.app/start/prerequisites/)
#
# Autor: Iago Souza
# ============================================================================

.PHONY: help install dev build build-frontend build-backend \
        build-linux build-windows build-macos build-all \
        check check-frontend check-backend lint fmt fmt-frontend fmt-backend \
        test clean clean-frontend clean-backend clean-all \
        icons info

# ----------------------------------------------------------------------------
# Variáveis
# ----------------------------------------------------------------------------

PNPM         := pnpm
CARGO        := cargo
TAURI_CLI    := $(PNPM) run tauri

# Diretórios
SRC_TAURI    := src-tauri
DIST_DIR     := dist
BUNDLE_DIR   := $(SRC_TAURI)/target/release/bundle

# Targets de compilação cruzada (Rust)
TARGET_LINUX_X86   := x86_64-unknown-linux-gnu
TARGET_LINUX_ARM   := aarch64-unknown-linux-gnu
TARGET_WINDOWS_GNU := x86_64-pc-windows-gnu
TARGET_MACOS_X86   := x86_64-apple-darwin
TARGET_MACOS_ARM   := aarch64-apple-darwin

# Cores para output (compatível com terminais ANSI)
COLOR_RESET  := \033[0m
COLOR_GREEN  := \033[1;32m
COLOR_YELLOW := \033[1;33m
COLOR_CYAN   := \033[1;36m
COLOR_RED    := \033[1;31m

# ============================================================================
# 📋 HELP — Comando padrão
# ============================================================================

## Exibe todos os comandos disponíveis
help:
	@echo ""
	@echo "$(COLOR_CYAN)╔══════════════════════════════════════════════════════════════╗$(COLOR_RESET)"
	@echo "$(COLOR_CYAN)║         🎯  Show de Revisão — Comandos Disponíveis          ║$(COLOR_RESET)"
	@echo "$(COLOR_CYAN)╚══════════════════════════════════════════════════════════════╝$(COLOR_RESET)"
	@echo ""
	@echo "$(COLOR_GREEN)  Desenvolvimento:$(COLOR_RESET)"
	@echo "    make install             Instala dependências (pnpm + cargo)"
	@echo "    make dev                 Inicia Tauri em modo desenvolvimento"
	@echo "    make dev-frontend        Inicia apenas o frontend (Vite)"
	@echo ""
	@echo "$(COLOR_GREEN)  Build — SO Atual:$(COLOR_RESET)"
	@echo "    make build               Build de release para o SO atual"
	@echo "    make build-frontend      Build apenas do frontend (Vite)"
	@echo "    make build-backend       Build apenas do backend (Cargo)"
	@echo "    make build-debug         Build de debug para o SO atual"
	@echo ""
	@echo "$(COLOR_GREEN)  Build — Por Sistema Operacional:$(COLOR_RESET)"
	@echo "    make build-linux         Build para Linux (x86_64)"
	@echo "    make build-linux-arm     Build para Linux (aarch64)"
	@echo "    make build-windows       Build para Windows (x86_64, .exe standalone)"
	@echo "    make build-macos         Build para macOS (x86_64, standalone)"
	@echo "    make build-macos-arm     Build para macOS (Apple Silicon, standalone)"
	@echo "    make build-all           Build para todos os SOs (requer toolchains)"
	@echo ""
	@echo "$(COLOR_GREEN)  Qualidade de Código:$(COLOR_RESET)"
	@echo "    make check               Verifica frontend (svelte-check) + backend"
	@echo "    make check-frontend      Verifica apenas o frontend"
	@echo "    make check-backend       Verifica apenas o backend (cargo check)"
	@echo "    make lint                Executa linters (clippy + svelte-check)"
	@echo "    make fmt                 Formata todo o código (prettier + rustfmt)"
	@echo "    make fmt-frontend        Formata apenas o frontend (prettier)"
	@echo "    make fmt-backend         Formata apenas o backend (rustfmt)"
	@echo ""
	@echo "$(COLOR_GREEN)  Testes:$(COLOR_RESET)"
	@echo "    make test                Executa todos os testes"
	@echo "    make test-backend        Executa testes do backend (cargo test)"
	@echo ""
	@echo "$(COLOR_GREEN)  Utilidades:$(COLOR_RESET)"
	@echo "    make clean               Limpa artefatos de build"
	@echo "    make clean-frontend      Limpa apenas o dist/ do frontend"
	@echo "    make clean-backend       Limpa apenas o target/ do backend"
	@echo "    make clean-all           Limpa tudo (inclui node_modules)"
	@echo "    make icons               Gera ícones a partir de um PNG base"
	@echo "    make info                Exibe informações do ambiente"
	@echo ""
	@echo "$(COLOR_YELLOW)  ⚠  Builds cross-platform requerem toolchains instalados.$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)     Recomenda-se usar GitHub Actions para CI/CD multi-SO.$(COLOR_RESET)"
	@echo ""

# ============================================================================
# 📦 INSTALAÇÃO
# ============================================================================

## Instala todas as dependências do projeto (frontend + backend)
install:
	@echo "$(COLOR_CYAN)📦 Instalando dependências do frontend (pnpm)...$(COLOR_RESET)"
	$(PNPM) install
	@echo "$(COLOR_CYAN)📦 Verificando dependências do backend (cargo)...$(COLOR_RESET)"
	cd $(SRC_TAURI) && $(CARGO) fetch
	@echo "$(COLOR_GREEN)✅ Dependências instaladas com sucesso!$(COLOR_RESET)"

# ============================================================================
# 🚀 DESENVOLVIMENTO
# ============================================================================

## Inicia o ambiente de desenvolvimento completo (Tauri + Vite + HMR)
dev:
	@echo "$(COLOR_CYAN)🚀 Iniciando Show de Revisão em modo desenvolvimento...$(COLOR_RESET)"
	$(TAURI_CLI) dev

## Inicia apenas o servidor de desenvolvimento do frontend (Vite)
dev-frontend:
	@echo "$(COLOR_CYAN)🚀 Iniciando frontend (Vite dev server)...$(COLOR_RESET)"
	$(PNPM) run dev

# ============================================================================
# 🔨 BUILD — SO ATUAL
# ============================================================================

## Build de release para o sistema operacional atual
build:
	@echo "$(COLOR_CYAN)🔨 Gerando build de release para o SO atual...$(COLOR_RESET)"
	$(TAURI_CLI) build
	@echo "$(COLOR_GREEN)✅ Build concluído! Verifique: $(BUNDLE_DIR)/$(COLOR_RESET)"

## Build apenas do frontend (Vite)
build-frontend:
	@echo "$(COLOR_CYAN)🔨 Gerando build do frontend (Vite)...$(COLOR_RESET)"
	$(PNPM) run build
	@echo "$(COLOR_GREEN)✅ Frontend compilado em $(DIST_DIR)/$(COLOR_RESET)"

## Build apenas do backend (Cargo release)
build-backend:
	@echo "$(COLOR_CYAN)🔨 Gerando build do backend (Cargo release)...$(COLOR_RESET)"
	cd $(SRC_TAURI) && $(CARGO) build --release
	@echo "$(COLOR_GREEN)✅ Backend compilado!$(COLOR_RESET)"

## Build de debug para o sistema operacional atual
build-debug:
	@echo "$(COLOR_CYAN)🔨 Gerando build de debug...$(COLOR_RESET)"
	$(TAURI_CLI) build --debug
	@echo "$(COLOR_GREEN)✅ Build de debug concluído!$(COLOR_RESET)"

# ============================================================================
# 🌍 BUILD — POR SISTEMA OPERACIONAL
# ============================================================================
# NOTA: Compilação cruzada (cross-compilation) com Tauri requer que as
# toolchains e dependências nativas do SO alvo estejam instaladas.
#
# Os targets de cross-compilation (Windows, macOS) usam --bundles none
# para gerar apenas o executável standalone (portátil), pois os bundlers
# nativos (NSIS/MSI, DMG) não funcionam fora do SO alvo.
#
# Vantagens do executável standalone:
#   - Zero instalação: baixou, executou
#   - Portátil: roda de pendrive
#   - Sem permissão de admin no computador
#
# Para instalar um target Rust:
#   rustup target add <target-triple>
#
# Para builds multiplataforma com instaladores nativos em CI/CD,
# recomenda-se GitHub Actions com runners nativos para cada SO. Veja:
#   https://v2.tauri.app/distribute/pipelines/
# ============================================================================

## Build para Linux x86_64 (DEB, RPM, AppImage)
build-linux:
	@echo "$(COLOR_CYAN)🐧 Gerando build para Linux (x86_64)...$(COLOR_RESET)"
	$(TAURI_CLI) build --target $(TARGET_LINUX_X86)
	@echo "$(COLOR_GREEN)✅ Build Linux x86_64 concluído!$(COLOR_RESET)"

## Build para Linux aarch64 / ARM64
build-linux-arm:
	@echo "$(COLOR_CYAN)🐧 Gerando build para Linux (aarch64)...$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)⚠  Requer: rustup target add $(TARGET_LINUX_ARM)$(COLOR_RESET)"
	$(TAURI_CLI) build --target $(TARGET_LINUX_ARM)
	@echo "$(COLOR_GREEN)✅ Build Linux ARM64 concluído!$(COLOR_RESET)"

## Build para Windows x86_64 — executável standalone (.exe portátil)
build-windows:
	@echo "$(COLOR_CYAN)🪟 Gerando build para Windows (x86_64) — standalone .exe...$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)⚠  Requer: rustup target add $(TARGET_WINDOWS_GNU)$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)⚠  Requer: mingw-w64 instalado (sudo apt install gcc-mingw-w64-x86-64)$(COLOR_RESET)"
	$(TAURI_CLI) build --target $(TARGET_WINDOWS_GNU) --no-bundle
	@echo "$(COLOR_GREEN)✅ Build Windows x86_64 concluído!$(COLOR_RESET)"
	@echo "$(COLOR_CYAN)📦 Executável em: $(SRC_TAURI)/target/$(TARGET_WINDOWS_GNU)/release/show-de-revisao.exe$(COLOR_RESET)"

## Build para macOS x86_64 (Intel) — executável standalone
build-macos:
	@echo "$(COLOR_CYAN)🍎 Gerando build para macOS (x86_64) — standalone...$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)⚠  Requer: rustup target add $(TARGET_MACOS_X86)$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)⚠  Cross-compile para macOS só funciona nativamente em macOS$(COLOR_RESET)"
	$(TAURI_CLI) build --target $(TARGET_MACOS_X86) --no-bundle
	@echo "$(COLOR_GREEN)✅ Build macOS x86_64 concluído!$(COLOR_RESET)"
	@echo "$(COLOR_CYAN)📦 Executável em: $(SRC_TAURI)/target/$(TARGET_MACOS_X86)/release/show-de-revisao$(COLOR_RESET)"

## Build para macOS aarch64 (Apple Silicon — M1/M2/M3/M4) — executável standalone
build-macos-arm:
	@echo "$(COLOR_CYAN)🍎 Gerando build para macOS (aarch64 / Apple Silicon) — standalone...$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)⚠  Requer: rustup target add $(TARGET_MACOS_ARM)$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)⚠  Cross-compile para macOS só funciona nativamente em macOS$(COLOR_RESET)"
	$(TAURI_CLI) build --target $(TARGET_MACOS_ARM) --no-bundle
	@echo "$(COLOR_GREEN)✅ Build macOS Apple Silicon concluído!$(COLOR_RESET)"
	@echo "$(COLOR_CYAN)📦 Executável em: $(SRC_TAURI)/target/$(TARGET_MACOS_ARM)/release/show-de-revisao$(COLOR_RESET)"

## Build para todos os sistemas operacionais (requer todas as toolchains)
build-all: build-linux build-windows build-macos build-macos-arm
	@echo ""
	@echo "$(COLOR_GREEN)══════════════════════════════════════════════════════════════$(COLOR_RESET)"
	@echo "$(COLOR_GREEN)  ✅ Builds para todos os SOs concluídos!$(COLOR_RESET)"
	@echo "$(COLOR_GREEN)══════════════════════════════════════════════════════════════$(COLOR_RESET)"
	@echo ""
	@echo "  Linux:          $(BUNDLE_DIR)/ (DEB, RPM, AppImage)"
	@echo "  Windows:        $(SRC_TAURI)/target/$(TARGET_WINDOWS_GNU)/release/show-de-revisao.exe"
	@echo "  macOS (Intel):  $(SRC_TAURI)/target/$(TARGET_MACOS_X86)/release/show-de-revisao"
	@echo "  macOS (ARM):    $(SRC_TAURI)/target/$(TARGET_MACOS_ARM)/release/show-de-revisao"
	@echo ""
	@echo "$(COLOR_YELLOW)  💡 Para CI/CD multiplataforma, considere usar GitHub Actions$(COLOR_RESET)"
	@echo "$(COLOR_YELLOW)     com runners nativos para cada SO.$(COLOR_RESET)"
	@echo ""

# ============================================================================
# ✅ VERIFICAÇÃO E QUALIDADE
# ============================================================================

## Verifica frontend (svelte-check) e backend (cargo check)
check: check-frontend check-backend
	@echo "$(COLOR_GREEN)✅ Todas as verificações passaram!$(COLOR_RESET)"

## Verifica apenas o frontend com svelte-check
check-frontend:
	@echo "$(COLOR_CYAN)🔍 Verificando frontend (svelte-check)...$(COLOR_RESET)"
	$(PNPM) exec svelte-check --tsconfig ./tsconfig.json

## Verifica apenas o backend com cargo check
check-backend:
	@echo "$(COLOR_CYAN)🔍 Verificando backend (cargo check)...$(COLOR_RESET)"
	cd $(SRC_TAURI) && $(CARGO) check

## Executa linters: clippy (Rust) + svelte-check (Svelte/TS)
lint: check-frontend
	@echo "$(COLOR_CYAN)🔍 Executando clippy (Rust)...$(COLOR_RESET)"
	cd $(SRC_TAURI) && $(CARGO) clippy -- -D warnings
	@echo "$(COLOR_GREEN)✅ Linting concluído sem warnings!$(COLOR_RESET)"

# ============================================================================
# 🎨 FORMATAÇÃO
# ============================================================================

## Formata todo o código (frontend + backend)
fmt: fmt-frontend fmt-backend
	@echo "$(COLOR_GREEN)✅ Todo o código formatado!$(COLOR_RESET)"

## Formata o frontend com Prettier
fmt-frontend:
	@echo "$(COLOR_CYAN)🎨 Formatando frontend (prettier)...$(COLOR_RESET)"
	$(PNPM) exec prettier --write "src/**/*.{svelte,ts,js,css,html}" \
		"*.html" "*.json" || \
		echo "$(COLOR_YELLOW)⚠  Prettier não encontrado. Instale: pnpm add -D prettier$(COLOR_RESET)"

## Formata o backend com rustfmt
fmt-backend:
	@echo "$(COLOR_CYAN)🎨 Formatando backend (rustfmt)...$(COLOR_RESET)"
	cd $(SRC_TAURI) && $(CARGO) fmt

# ============================================================================
# 🧪 TESTES
# ============================================================================

## Executa todos os testes (backend)
test: test-backend
	@echo "$(COLOR_GREEN)✅ Todos os testes passaram!$(COLOR_RESET)"

## Executa testes do backend (cargo test)
test-backend:
	@echo "$(COLOR_CYAN)🧪 Executando testes do backend (cargo test)...$(COLOR_RESET)"
	cd $(SRC_TAURI) && $(CARGO) test

# ============================================================================
# 🧹 LIMPEZA
# ============================================================================

## Limpa artefatos de build (dist/ + target/release/bundle/)
clean: clean-frontend clean-backend
	@echo "$(COLOR_GREEN)✅ Artefatos de build limpos!$(COLOR_RESET)"

## Limpa apenas o diretório dist/ do frontend
clean-frontend:
	@echo "$(COLOR_CYAN)🧹 Limpando frontend (dist/)...$(COLOR_RESET)"
	rm -rf $(DIST_DIR)

## Limpa apenas o diretório target/ do backend
clean-backend:
	@echo "$(COLOR_CYAN)🧹 Limpando backend (target/)...$(COLOR_RESET)"
	cd $(SRC_TAURI) && $(CARGO) clean

## Limpa TUDO: dist/, target/ e node_modules/
clean-all: clean
	@echo "$(COLOR_RED)🧹 Removendo node_modules/...$(COLOR_RESET)"
	rm -rf node_modules
	@echo "$(COLOR_GREEN)✅ Limpeza completa realizada!$(COLOR_RESET)"

# ============================================================================
# 🛠️ UTILIDADES
# ============================================================================

## Gera ícones do aplicativo a partir de um PNG base (requer app-icon.png 1024x1024)
icons:
	@echo "$(COLOR_CYAN)🎨 Gerando ícones do aplicativo...$(COLOR_RESET)"
	@if [ ! -f app-icon.png ]; then \
		echo "$(COLOR_RED)❌ Arquivo app-icon.png não encontrado na raiz do projeto!$(COLOR_RESET)"; \
		echo "$(COLOR_YELLOW)   Coloque um PNG 1024x1024 chamado app-icon.png na raiz.$(COLOR_RESET)"; \
		exit 1; \
	fi
	$(TAURI_CLI) icon app-icon.png
	@echo "$(COLOR_GREEN)✅ Ícones gerados em $(SRC_TAURI)/icons/$(COLOR_RESET)"

## Exibe informações sobre o ambiente de desenvolvimento
info:
	@echo ""
	@echo "$(COLOR_CYAN)╔══════════════════════════════════════════════════════════════╗$(COLOR_RESET)"
	@echo "$(COLOR_CYAN)║           🔧  Informações do Ambiente                       ║$(COLOR_RESET)"
	@echo "$(COLOR_CYAN)╚══════════════════════════════════════════════════════════════╝$(COLOR_RESET)"
	@echo ""
	@echo "  SO:           $$(uname -sr)"
	@echo "  Arch:         $$(uname -m)"
	@echo "  Shell:        $$(basename $$SHELL)"
	@echo ""
	@echo "  Node.js:      $$(node --version 2>/dev/null || echo 'não instalado')"
	@echo "  pnpm:         $$(pnpm --version 2>/dev/null || echo 'não instalado')"
	@echo "  Rust:         $$(rustc --version 2>/dev/null || echo 'não instalado')"
	@echo "  Cargo:        $$(cargo --version 2>/dev/null || echo 'não instalado')"
	@echo "  Tauri CLI:    $$(pnpm tauri --version 2>/dev/null || echo 'não instalado')"
	@echo ""
	@echo "  Targets Rust instalados:"
	@rustup target list --installed 2>/dev/null | sed 's/^/    - /' || echo "    rustup não encontrado"
	@echo ""
