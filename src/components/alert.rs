use leptos::*;

use crate::stores::{default::DefaultState, kegiatan::KegiatanState};

#[component]
pub fn SuccessAlert() -> impl IntoView {
    let state = expect_context::<RwSignal<DefaultState>>();
    let (_, set_success) =
        create_slice(state, |st| st.successalert, |st, val| st.successalert = val);

    let kegiatanstate = expect_context::<RwSignal<KegiatanState>>();
    let (pesanalert, set_pesanalert) = create_slice(
        kegiatanstate,
        |st| st.pesanalert.clone(),
        |st, val| st.pesanalert = val,
    );

    let onbtnclick = move |_| {
        set_success(false);
        set_pesanalert("".to_string())
    };

    view! {
        <div role="alert" class="rounded-xl border border-gray-200 dark:border-gray-500 bg-white dark:bg-gray-600 p-4">
            <div class="flex items-start gap-4">
                <span class="text-green-600">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="h-6 w-6"
                    >
                        <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                        />
                    </svg>
                </span>

                <div class="flex-1">
                    <strong class="block font-medium text-gray-900 dark:text-white">"Perubahan disimpan"</strong>

                    <p class="mt-1 text-sm text-gray-700 dark:text-gray-400">
                        "Rekaman data"
                        <span class="whitespace-nowrap rounded-full bg-purple-100 px-2.5 py-0.5 text-sm text-purple-700 mx-2">
                            {pesanalert}
                        </span>
                        "berhasil disimpan."
                    </p>
                </div>

                <button
                    class="text-gray-500 dark:text-gray-300 transition hover:text-gray-600 dark:hover:text-gray-400"
                    on:click=onbtnclick
                >
                    <span class="sr-only">"Dismiss popup"</span>

                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="h-6 w-6"
                    >
                        <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M6 18L18 6M6 6l12 12"
                        />
                    </svg>
                </button>
            </div>
        </div>
    }
}
