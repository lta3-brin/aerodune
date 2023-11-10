mod app;
mod pages;
mod stores;
mod models;
mod configs;
mod components;

use leptos::*;

use crate::app::default::DefaultApp;

fn main() {
    mount_to_body(|| {
        view! {
            <DefaultApp />
        }
    })
}
