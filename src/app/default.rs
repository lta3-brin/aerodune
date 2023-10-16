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
        <Show
            when=light
            fallback=|| view! {
                <Html lang="en" attr:data-theme="dark" />
            }
        >
            <Html lang="en" attr:data-theme="light" />
        </Show>
        <div class="flex flex-row sm:gap-10">
            <div class="sm:w-full sm:max-w-[18rem]">
                <input type="checkbox" id="sidebar-mobile-fixed" class="sidebar-state" />
                <label for="sidebar-mobile-fixed" class="sidebar-overlay"></label>
                <aside class="sidebar sidebar-fixed-left sidebar-mobile h-full justify-start max-sm:fixed max-sm:-translate-x-full">
                    <DefaultBrand />
                    <DefaultSidebar />
                    <DefaultSideFooter />
                </aside>
            </div>
            <div class="flex w-full flex-col p-4">
                <div class="flex space-x-2">
                    <label for="sidebar-mobile-fixed" class="btn btn-outline btn-sm sm:hidden">
                        <svg class="w-4 h-4 text-gray-400 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15"/>
                        </svg>
                    </label>
                </div>

                <div class="my-4 grid grid-cols-2 gap-4">
                    <HalamanUtama />
                </div>
            </div>
        </div>
    }
}
