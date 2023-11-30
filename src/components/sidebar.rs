use leptos::*;
use leptos_router::*;

use crate::{models::kegiatan::KegiatanTambah, stores::default::DefaultState};

#[component]
pub fn DefaultSidebar() -> impl IntoView {
    view! {
        <div class="w-60 flex h-screen flex-col justify-between border-e bg-white dark:bg-gray-800 dark:border-gray-700">
            <div class="px-4 py-6">
                <DefaultBrand />
                <DefaultSideMenu />
            </div>

            <div class="sticky inset-x-0 bottom-0 border-t border-gray-100 dark:border-gray-700">
                <DefaultSideFooter />
            </div>
        </div>
    }
}

#[component]
pub fn DefaultBrand() -> impl IntoView {
    view! {
        <div class="flex place-items-center">
            <div class="flex-none">
                <img src="public/logo.png" class="w-8 mr-2" alt="Logo aplikasi" />
            </div>
            <div class="flex-1">
                <h1 class="uppercase text-gray-700 font-bold dark:text-gray-300">Dashboard</h1>
            </div>
        </div>
    }
}

#[component]
pub fn DefaultSideMenu() -> impl IntoView {
    let state = expect_context::<RwSignal<DefaultState>>();
    let (pg, _) = create_slice(state, |st| st.page, |st, val| st.page = val);

    let src = create_local_resource(pg, KegiatanTambah::muat);

    let kegiatan = move || {
        src.and_then(|data| {
            data.iter()
                .map(|_keg| {
                    view! {
                        <li>
                            <A href="/kegiatan/2" exact=true class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 dark:hover:text-gray-300">
                                "ABC"
                            </A>
                        </li>
                    }
                })
                .collect_view()
        })
    };

    let fallback = move |errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| {
                        view! {
                            <li class="mt-2 text-sm text-red-700">
                                {e.to_string()}
                            </li>
                        }
                    })
                    .collect_view()
            })
        };

        view! {
            <div role="alert" class="rounded border-s-4 border-red-500 bg-red-50 p-4">
                <strong class="block font-medium text-red-800">
                    "Terjadi kesalahan"
                </strong>

                <ul>{error_list}</ul>
            </div>
        }
    };

    view! {
        <ul class="mt-6 space-y-1">
            <Suspense fallback=move || view! { <li>"Memuat kegiatan"</li> }>
                <ErrorBoundary fallback>
                    { kegiatan }
                </ErrorBoundary>
            </Suspense>
        </ul>
    }
}

#[component]
pub fn DefaultSideFooter() -> impl IntoView {
    view! {
        <A href="/kegiatan" exact=true class="block px-12 py-3 text-center text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:hover:bg-gray-600 dark:hover:text-gray-300 focus:outline-none focus:ring active:bg-gray-100 dark:text-gray-300">
            Tambah Kegiatan
        </A>
    }
}
