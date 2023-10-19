#[derive(Debug, Clone)]
pub struct DefaultState {
    pub light: bool,
    pub closesidebar: bool,
}

impl Default for DefaultState {
    fn default() -> Self {
        Self {
            light: true,
            closesidebar: true,
        }
    }
}
