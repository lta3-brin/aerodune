// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod instruksi;
mod model;
mod store;

use anyhow::Result;
use std::env;
use surrealdb::engine::local::File;

use crate::instruksi::kegiatan::greet;
use crate::store::persistent::DB;

#[tokio::main]
async fn main() -> Result<()> {
    let path = env::current_dir()?;
    let dir = format!("{}/data", path.display());
    DB.connect::<File>(dir).await?;
    DB.use_ns("aerodune").use_db("kalibrasi").await?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
