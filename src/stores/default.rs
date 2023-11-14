#[derive(Debug)]
pub struct DefaultState {
    pub light: bool,
    pub closesidebar: bool,
    pub successalert: bool,
    pub failalert: bool,
    pub page: u32,
}

impl Default for DefaultState {
    fn default() -> Self {
        Self {
            light: false,
            closesidebar: true,
            successalert: false,
            failalert: false,
            page: 1
        }
    }
}
