use leptos::*;

#[component]
pub fn DefaultBrand() -> impl IntoView {
    view! {
        <span class="grid h-10 w-32 place-content-center rounded-lg bg-gray-100 text-xs text-gray-600">
            Logo
            <img src="public/logo.png" class="w-8 mr-2" alt="Logo aplikasi" />
        </span>
    }
}
