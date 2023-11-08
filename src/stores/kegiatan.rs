#[derive(Debug, Clone)]
pub struct KegiatanState {
    pub pesanalert: String,
}

impl Default for KegiatanState {
    fn default() -> Self {
        Self {
            pesanalert: "".to_string(),
        }
    }
}
