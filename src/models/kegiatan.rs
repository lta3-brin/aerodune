use chrono::prelude::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct KegiatanMuat {
    pub id: Thing,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KegiatanTambah {
    pub name: String,
    pub create: DateTime<Local>,
    pub update: DateTime<Local>,
}
