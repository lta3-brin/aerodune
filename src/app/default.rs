use leptos::*;
use leptos_meta::*;

use crate::pages::utama::HalamanUtama;

#[component]
pub fn DefaultApp() -> impl IntoView {
    provide_meta_context();

    view! {
        <HalamanUtama />
        <Script src="public/flowbite.min.js"></Script>
    }
}
