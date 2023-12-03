use chrono::prelude::Local;
use leptos::error::Result;
use surrealdb::engine::remote::ws::Ws;

use crate::configs::db::DB;
use crate::configs::general::PAGE_LIMIT;
use crate::models::kegiatan::KegiatanMuat;
use crate::models::{kegiatan::KegiatanTambah, Record};

impl KegiatanTambah {
    pub async fn init() -> Result<Vec<KegiatanMuat>> {
        DB.connect::<Ws>("localhost:8000").await?;
        DB.use_ns("aerodune").use_db("kalibrasi").await?;

        let mut q = DB
            .query("select * from kegiatan order by create desc limit $limit")
            .bind(("limit", PAGE_LIMIT))
            .await?;

        let kegiatan: Vec<KegiatanMuat> = q.take(0)?;

        Ok(kegiatan)
    }

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

    pub async fn _muat(page: u32) -> Result<Vec<KegiatanMuat>> {
        let offset = PAGE_LIMIT * (page - 1);
        let mut q = DB
            .query("select * from kegiatan order by create desc limit $limit start $start")
            .bind(("limit", PAGE_LIMIT))
            .bind(("start", offset))
            .await?;

        let kegiatan: Vec<KegiatanMuat> = q.take(0)?;

        Ok(kegiatan)
    }
}
