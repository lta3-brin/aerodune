use leptos::*;

use crate::stores::default::DefaultState;
#[component]
pub fn DefaultBtns() -> impl IntoView {
    let state = expect_context::<RwSignal<DefaultState>>();
    let (light, set_light) = create_slice(state, |st| st.light, |st, val| st.light = val);
    let (side, set_sidebar) =
        create_slice(state, |st| st.closesidebar, |st, val| st.closesidebar = val);

    let onclicktema = move |_| {
        set_light(!light());
    };

    let onclicksidebar = move |_| {
        set_sidebar(!side());
    };

    view! {
        <div class="inline-flex rounded-lg border border-gray-100 dark:border-gray-900 bg-gray-100 dark:bg-gray-600 p-1">
            <button
                class="inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm text-gray-500 dark:text-gray-300 dark:hover:text-blue-300 hover:text-blue-500 hover:shadow-sm hover:bg-white dark:hover:bg-gray-500 focus:relative"
            >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="h-4 w-4"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10"
                    />
                </svg>
            </button>

            <button
                class="inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm text-gray-500 dark:text-gray-300 dark:hover:text-blue-300 hover:text-blue-500 hover:shadow-sm hover:bg-white dark:hover:bg-gray-500 focus:relative"
                on:click=onclicktema
            >
                <Show when=light fallback=|| view! {
                    <svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" />
                    </svg>
                }>
                    <svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z" />
                    </svg>
                </Show>
            </button>

            <button
                class="lg:hidden inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm text-gray-500 dark:text-gray-300 dark:hover:text-blue-300 hover:text-blue-500 hover:shadow-sm hover:bg-white dark:hover:bg-gray-500 focus:relative"
                on:click=onclicksidebar
            >
                <Show when=side fallback=|| view! {
                    <svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                }>
                    <svg class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 5.25h16.5m-16.5 4.5h16.5m-16.5 4.5h16.5m-16.5 4.5h16.5" />
                    </svg>
                </Show>
            </button>
        </div>
    }
}

