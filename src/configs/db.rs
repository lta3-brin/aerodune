use once_cell::sync::Lazy;
use surrealdb::{Surreal, engine::remote::ws::Client};

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);