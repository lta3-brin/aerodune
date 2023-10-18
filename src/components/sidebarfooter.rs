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
        <a href="#" class="flex items-center gap-2 bg-white p-4 hover:bg-gray-50" on:click=onclick>
            <img
                alt="Man"
                src="https://images.unsplash.com/photo-1600486913747-55e5470d6f40?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1770&q=80"
                class="h-10 w-10 rounded-full object-cover"
            />

            <div>
                <p class="text-xs">
                <strong class="block font-medium">Eric Frusciante</strong>

                <span> eric@frusciante.com </span>
                </p>
            </div>
        </a>
    }
}
