use leptos::*;

use crate::stores::default::DefaultState;

#[component]
pub fn DefaultSideFooter() -> impl IntoView {
    let state = expect_context::<RwSignal<DefaultState>>();
    let (light, set_light) = create_slice(state, |st| st.light, |st, val| st.light = val);
    let onclick = move |_| {
        set_light(!light());
    };

    view! {
        <section class="sidebar-footer justify-end bg-gray-2 pt-2">
            <div class="divider my-0"></div>
            <div class="dropdown z-50 flex h-fit w-full cursor-pointer hover:bg-gray-4">
                <label class="whites mx-2 flex h-fit w-full cursor-pointer p-0 hover:bg-gray-4" tabindex="0">
                    <div class="flex flex-row gap-4 p-4">
                        <svg class="w-4 h-4 opacity-75" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor" viewBox="0 0 20 20">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 12.25V1m0 11.25a2.25 2.25 0 0 0 0 4.5m0-4.5a2.25 2.25 0 0 1 0 4.5M4 19v-2.25m6-13.5V1m0 2.25a2.25 2.25 0 0 0 0 4.5m0-4.5a2.25 2.25 0 0 1 0 4.5M10 19V7.75m6 4.5V1m0 11.25a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5ZM16 19v-2"/>
                        </svg>
                        <div class="flex flex-col">
                            <span>"Pengaturan"</span>
                        </div>
                    </div>
                </label>
                <div class="dropdown-menu-top-center md:dropdown-menu-right-top dropdown-menu ml-2">
                    <a class="dropdown-item text-sm">"Tambah Kegiatan"</a>
                    <a tabindex="-1" class="dropdown-item text-sm" on:click=onclick>"Ganti Tampilan"</a>
                </div>
            </div>
        </section>
    }
}
