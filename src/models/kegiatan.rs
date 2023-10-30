use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KegiatanArgs<'a> {
    pub name: &'a str,
}
