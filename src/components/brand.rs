use leptos::*;

#[component]
pub fn DefaultBrand() -> impl IntoView {
    view! {
        <div class="flex place-items-center">
            <div class="flex-none">
                <img src="public/logo.png" class="w-8 mr-2" alt="Logo aplikasi" />
            </div>
            <div class="flex-1">
                <h1 class="uppercase text-gray-700 font-bold">Dashboard</h1>
            </div>
        </div>
    }
}

