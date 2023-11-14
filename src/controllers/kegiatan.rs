use anyhow::Result;
use chrono::prelude::Local;

use crate::configs::db::DB;
use crate::configs::general::PAGE_LIMIT;
use crate::models::kegiatan::KegiatanMuat;
use crate::models::{kegiatan::KegiatanTambah, Record};

impl KegiatanTambah {
    pub async fn tambah(name: String) -> Result<Vec<Record>> {
        let created: Vec<Record> = DB
            .create("kegiatan")
            .content(Self {
                name,
                create: Local::now(),
                update: Local::now(),
            })
            .await?;

        Ok(created)
    }

    pub async fn muat(page: u32) -> Vec<KegiatanMuat> {
        let offset = PAGE_LIMIT * (page - 1);
        let q = DB
            .query("select * from kegiatan order by create desc limit $limit start $start")
            .bind(("limit", PAGE_LIMIT))
            .bind(("start", offset))
            .await;

        match q {
            Ok(mut res) => {
                let kegiatan: Vec<KegiatanMuat> = res.take(0).unwrap_or_default();

                kegiatan
            }
            Err(_) => {
                vec![]
            }
        }
    }
}
