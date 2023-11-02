use once_cell::sync::Lazy;
use surrealdb::{engine::local::Db, Surreal};

pub static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

