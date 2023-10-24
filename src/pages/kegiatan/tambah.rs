use leptos::*;
use leptos_router::*;

#[component]
pub fn TambahKegiatan() -> impl IntoView {
    let query = use_query_map();
    let kegiatan = move || query().get("kegiatan").cloned().unwrap_or_default();

    view! {
        <Form method="GET" action="" class="grid grid-cols-1 md:grid-cols-6 gap-3">
            <div class="md:col-span-5">
                <label
                    for="Username"
                    class="relative block rounded-md border border-gray-200 shadow-sm focus-within:border-blue-600 focus-within:ring-1 focus-within:ring-blue-600"
                >
                    <input
                        type="text"
                        id="kegiatan"
                        class="w-full peer border-none bg-transparent placeholder-transparent focus:border-transparent focus:outline-none focus:ring-0"
                        placeholder="Kegiatan"
                        name="kegiatan"
                        value=kegiatan
                    />

                    <span
                        class="pointer-events-none absolute start-2.5 top-0 -translate-y-1/2 bg-white dark:bg-gray-800 p-0.5 text-xs text-gray-700 dark:text-gray-300 transition-all peer-placeholder-shown:top-1/2 peer-placeholder-shown:text-sm peer-focus:top-0 peer-focus:text-xs"
                    >
                        "Nama kegiatan"
                    </span>
                </label>
            </div>
            <div>
                <button type="submit"
                    class="w-full px-5 py-3 text-sm uppercase font-medium text-white bg-indigo-600 rounded hover:bg-indigo-700 focus:outline-none focus:ring"
                >
                    "Simpan"
                </button>
            </div>
        </Form>
    }
}
