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
        <a href="/" class="block border-y-gray-100 px-12 py-3 text-center text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700 focus:outline-none focus:ring active:bg-gray-100" on:click=onclick>
            Tambah Kegiatan
        </a>
    }
}

