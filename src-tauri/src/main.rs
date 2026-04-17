// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Caminho relativo: src-tauri/src/main.rs
/// \file src-tauri/src/main.rs
/// \brief Application entry point
/// \author Iago Souza

fn main() {
    show_de_revisao_lib::run()
}
