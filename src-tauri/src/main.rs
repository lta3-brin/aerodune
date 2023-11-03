// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod instruksi;
mod model;
mod store;

use anyhow::Result;
use std::env;
use surrealdb::engine::remote::ws::Ws;

use crate::instruksi::kegiatan::tambahkegiatan;
use crate::store::persistent::DB;

#[tokio::main]
async fn main() -> Result<()> {
    DB.connect::<Ws>("localhost:8000").await?;
    DB.use_ns("aerodune").use_db("kalibrasi").await?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tambahkegiatan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

