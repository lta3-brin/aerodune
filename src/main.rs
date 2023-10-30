mod app;
mod components;
mod models;
mod pages;
mod stores;

use leptos::*;

use crate::app::default::DefaultApp;

fn main() {
    mount_to_body(|| {
        view! {
            <DefaultApp />
        }
    })
}
