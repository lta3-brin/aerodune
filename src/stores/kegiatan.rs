#[derive(Debug, Clone)]
pub struct KegiatanState {
    pub pesanalert: String,
    pub kegiatanpage: u32,
}

impl Default for KegiatanState {
    fn default() -> Self {
        Self {
            pesanalert: "".to_string(),
            kegiatanpage: 0,
        }
    }
}
