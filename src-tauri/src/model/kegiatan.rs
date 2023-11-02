use chrono::prelude::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KegiatanArgs {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KegiatanTambah {
    pub name: String,
    pub create: DateTime<Local>,
    pub update: DateTime<Local>,
}
