use leptos::*;

#[component]
pub fn HalamanUtama() -> impl IntoView {
    view! {
        <section class="dark:bg-gray-800">
            <div
                class="mx-auto max-w-screen-xl px-4 py-32 lg:flex lg:items-center"
            >
                <div class="mx-auto max-w-xl text-center">
                    <h1 class="text-3xl font-extrabold sm:text-5xl">
                        <strong class="font-extrabold text-purple-700 sm:block">
                            "Kalibrasi Instrumentasi"
                        </strong>
                        "LA3 BRIN"
                    </h1>

                    <p class="mt-4 sm:text-xl/relaxed">
                        "Silahkan pilih kegiatan pada menu samping."
                    </p>
                </div>
            </div>
            </section>
    }
}

