use leptos::*;
use leptos_meta::*;

use crate::components::{
    buttongroups::DefaultBtns,
    sidebar::{DefaultBrand, DefaultSideFooter, DefaultSideMenu, DefaultSidebar},
};
use crate::pages::utama::HalamanUtama;
use crate::stores::default::DefaultState;

#[component]
pub fn DefaultApp() -> impl IntoView {
    provide_meta_context();
    provide_context(create_rw_signal(DefaultState::default()));

    let state = expect_context::<RwSignal<DefaultState>>();
    let (light, _) = create_slice(state, |st| st.light, |st, val| st.light = val);
    let (side, set_side) =
        create_slice(state, |st| st.closesidebar, |st, val| st.closesidebar = val);

    let onclicksidebar = move |_| {
        set_side(true);
    };

    view! {
        <Show when=light fallback=|| view! {
            <Html lang="en" class="dark bg-gray-800" />
        }>
            <Html lang="en" class="light" />
        </Show>

        <div class="grid grid-cols-1 lg:grid-cols-5 dark:bg-gray-800 dark:text-gray-300">
            <div class="hidden lg:flex h-screen flex-col justify-between border-e bg-white dark:bg-gray-800 dark:border-gray-700">
                <div class="px-4 py-6">
                    <DefaultBrand />
                    <DefaultSideMenu />
                </div>

                <div class="sticky inset-x-0 bottom-0 border-t border-gray-100 dark:border-gray-700">
                    <DefaultSideFooter />
                </div>
            </div>

            <div class="p-7 lg:col-span-4">
                <div class="flex flex-row-reverse mb-3">
                    <DefaultBtns />
                </div>
                <HalamanUtama />
            </div>
        </div>

        <div class="absolute top-0 left-0 z-50 lg:hidden"
            class:hidden=side
        >
            <DefaultSidebar />
        </div>
        <div
            class="w-screen h-screen absolute top-0 left-0 z-20 bg-gray-700 opacity-70 lg:hidden"
            class:hidden=side
            on:click=onclicksidebar
        />
    }
}
