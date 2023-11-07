use crate::app::default::invoke;
use berbagi::models::kegiatan::KegiatanArgs;
use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde_wasm_bindgen::to_value;

#[component]
pub fn TambahKegiatan() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());

    let simpankegiatan = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name().is_empty() {
                return;
            }

            let args = to_value(&KegiatanArgs { name: name() }).unwrap();
            let new_msg = invoke("tambahkegiatan", args).await.as_string().unwrap();
            set_name("".to_string());
        });
    };

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name(v);
    };

    view! {
        <form class="grid grid-cols-1 md:grid-cols-6 gap-3" on:submit=simpankegiatan>
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
                        prop:value=name
                        on:input=update_name
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
        </form>
    }
}
