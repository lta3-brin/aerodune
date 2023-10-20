use leptos::*;
use leptos_router::*;

#[component]
pub fn TakDitemukan() -> impl IntoView {
    view! {
        <div class="grid px-4 bg-white place-content-center dark:bg-gray-800">
            <div class="text-center">
                <h1 class="font-black text-gray-800 dark:text-gray-200 text-9xl">"404"</h1>

                <p class="text-2xl font-bold tracking-tight text-gray-800 dark:text-gray-300 sm:text-4xl">
                    "Eits, Maaf ya 🙏🏽"
                </p>

                <p class="mt-4 text-gray-500">
                    "Halaman ini tidak ditemukan."
                </p>

                <A
                    href="/"
                    class="inline-block px-5 py-3 mt-6 text-sm font-medium text-white bg-indigo-600 rounded hover:bg-indigo-700 focus:outline-none focus:ring"
                >
                    "Kembali ke halaman utama"
                </A>
            </div>
        </div>
    }
}

