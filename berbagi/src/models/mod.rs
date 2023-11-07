pub mod kegiatan;

use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}
