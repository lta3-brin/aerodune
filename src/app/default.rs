use leptos::*;
use leptos_meta::*;

use crate::components::{
    brand::DefaultBrand, sidebar::DefaultSidebar, sidebarfooter::DefaultSideFooter,
};
use crate::pages::utama::HalamanUtama;
use crate::stores::default::DefaultState;

#[component]
pub fn DefaultApp() -> impl IntoView {
    provide_context(create_rw_signal(DefaultState::default()));
    provide_meta_context();

    let state = expect_context::<RwSignal<DefaultState>>();
    let (light, _) = create_slice(state, |st| st.light, |st, val| st.light = val);

    view! {
        <Show when=light fallback=|| view! {
            <Html lang="en" class="light" />
        }>
            <Html lang="en" class="dark" />
        </Show>

        <div class="flex h-screen flex-col justify-between border-e bg-white">
            <div class="px-4 py-6">
                <DefaultBrand />
                <DefaultSidebar />
            </div>

            <div class="sticky inset-x-0 bottom-0 border-t border-gray-100">
                <DefaultSideFooter />
            </div>
        </div>

        <HalamanUtama />
    }
}
