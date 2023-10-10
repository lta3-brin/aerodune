mod app;
mod pages;

use leptos::*;

use crate::app::default::DefaultApp;

fn main() {
    mount_to_body(|| {
        view! {
            <DefaultApp />
        }
    })
}
