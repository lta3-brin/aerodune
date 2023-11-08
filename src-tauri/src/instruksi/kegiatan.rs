use chrono::prelude::Local;

use crate::store::persistent::DB;
use berbagi::models::kegiatan::KegiatanTambah;
use berbagi::models::Record;

#[tauri::command]
pub async fn tambahkegiatan(name: String) -> Result<String, String> {
    let created: Result<Vec<Record>, surrealdb::Error> = DB
        .create("kegiatan")
        .content(KegiatanTambah {
            name,
            create: Local::now(),
            update: Local::now(),
        })
        .await;

    match created {
        Ok(data) => {
            let kegiatan = data.first();
            if let Some(keg) = kegiatan {
                Ok(format!("#{}", keg.id))
            } else {
                Ok("ID kegiatan tidak ditemukan saat penyimpanan.".to_string())
            }
        }

        Err(_) => {
            let msg = "Kegiatan belum berhasil disimpan.".to_string();

            Err(msg)
        }
    }
}

#[tauri::command]
pub async fn muatkegiatan(page: u32) -> Result<String, String> {
    let limit = 10;
    let mut start = 0;

    if page > 0 {
        start = (page - 1) * limit + 1;
    }

    let response = DB
        .query("SELECT * FROM kegiatan ORDER BY create DESC LIMIT $limit START $start")
        .bind(("limit", limit))
        .bind(("start", start))
        .await;

    Ok(String::from("COBA"))
}
