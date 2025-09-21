// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod database;
mod commands;

use database::DatabaseManager;
use std::sync::Mutex;
use tauri_plugin_dialog;

fn main() {
    // Initialize database manager
    let db_manager = Mutex::new(DatabaseManager::new());
    
    tauri::Builder::default()
        // .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(db_manager)
        .invoke_handler(tauri::generate_handler![
            commands::test_connection,
            commands::connect_database,
            commands::get_database_tables,
            commands::get_table_data,
            commands::compare_database_schemas,
        ])
        .run(tauri::generate_context!())
                .expect("error while running tauri application");
         
        }

        