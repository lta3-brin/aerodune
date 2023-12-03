use leptos::*;
use leptos_router::*;

use crate::{models::kegiatan::KegiatanTambah, controllers::alert::default_fallback};

#[component]
pub fn DefaultSidebar() -> impl IntoView {
    view! {
        <div class="w-60 flex h-screen flex-col justify-between border-e bg-white dark:bg-gray-800 dark:border-gray-700">
            <div class="overflow-y-auto px-4 py-6">
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
    let src = create_local_resource(move || (), |_| KegiatanTambah::init());

    let kegiatan_awal = move || {
        src.and_then(|data| {
            data.iter()
                .map(|keg| {
                    let kg = keg.clone();
                    let id = kg.id.id.to_string();
                    let url = format!("/kegiatan/{id}");

                    view! {
                        <li>
                            <A href=url exact=true class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700 dark:text-gray-300 dark:hover:bg-gray-600 dark:hover:text-gray-300">
                                { kg.name }
                            </A>
                        </li>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <ul class="mt-6 space-y-1">
            <Suspense fallback=move || view! { 
                <div role="status" class="max-w-sm animate-pulse">
                    <div class="h-2.5 bg-gray-200 rounded-full dark:bg-gray-700 w-100 mb-4"></div>
                    <div class="h-2 bg-gray-200 rounded-full dark:bg-gray-700 max-w-[360px] mb-2.5"></div>
                    <div class="h-2 bg-gray-200 rounded-full dark:bg-gray-700 mb-2.5"></div>
                    <div class="h-2 bg-gray-200 rounded-full dark:bg-gray-700 max-w-[330px] mb-2.5"></div>
                    <div class="h-2 bg-gray-200 rounded-full dark:bg-gray-700 max-w-[300px] mb-2.5"></div>
                    <div class="h-2 bg-gray-200 rounded-full dark:bg-gray-700 max-w-[360px]"></div>
                    <span class="sr-only">"Memuat kegiatan..."</span>
                </div>
             }>
                <ErrorBoundary fallback=default_fallback>
                    { kegiatan_awal }
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
