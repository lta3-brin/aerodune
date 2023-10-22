use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::galat::TakDitemukan;
use crate::components::{buttongroups::DefaultBtns, sidebar::DefaultSidebar};
use crate::pages::utama::HalamanUtama;
use crate::stores::default::DefaultState;

#[component]
pub fn DefaultApp() -> impl IntoView {
    provide_meta_context();
    provide_context(create_rw_signal(DefaultState::default()));

    let state = expect_context::<RwSignal<DefaultState>>();
    let (light, _) = create_slice(state, |st| st.light, |st, val| st.light = val);
    let (side, _) = create_slice(state, |st| st.closesidebar, |st, val| st.closesidebar = val);

    view! {
        <Show when=light fallback=|| view! {
            <Html lang="en" class="dark bg-gray-800" />
        }>
            <Html lang="en" class="light" />
        </Show>

        <Router>
            <div class="p-7 lg:ml-60">
                <div class="flex flex-row-reverse mb-3">
                    <DefaultBtns />
                </div>

                <Routes>
                    <Route path="/" view=HalamanUtama />
                    <Route path="/*any" view=TakDitemukan />
                </Routes>
            </div>
        </Router>

        <div class="absolute top-0 left-0 invisible lg:visible">
            <DefaultSidebar />
        </div>

        <div class="absolute top-0 left-0 z-50 hidden"
            class:hidden=side
        >
            <DefaultSidebar />
        </div>
    }
}

