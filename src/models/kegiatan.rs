use chrono::prelude::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KegiatanMuat {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KegiatanTambah {
    pub name: String,
    pub create: DateTime<Local>,
    pub update: DateTime<Local>,
}
