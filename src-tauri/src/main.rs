// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod instruksi;
mod model;
mod store;

use std::env;
use surrealdb::engine::local::File;
use surrealdb::Surreal;

use crate::instruksi::kegiatan::greet;

#[tokio::main]
async fn main() {
    let path = env::current_dir().unwrap();
    let dir = format!("{}/data", path.display());
    let _db = Surreal::new::<File>(dir).await.unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
