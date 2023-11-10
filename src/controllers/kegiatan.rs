use anyhow::Result;
use chrono::prelude::Local;

use crate::configs::db::DB;
use crate::models::kegiatan::KegiatanTambah;

impl KegiatanTambah {
    pub async fn tambah(name: String) -> Result<Vec<Self>> {
        let created: Vec<Self> = DB
            .create("kegiatan")
            .content(Self {
                name,
                create: Local::now(),
                update: Local::now(),
            })
            .await?;

        Ok(created)
    }
}
