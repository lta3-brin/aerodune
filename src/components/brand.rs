use leptos::*;

#[component]
pub fn DefaultBrand() -> impl IntoView {
    view! {
        <section class="sidebar-title items-center p-4">
            <img src="public/logo.png" class="w-10 mr-2" alt="Logo aplikasi" />
            <div class="flex flex-col">
                <span>"Dashboard"</span>
                <span class="text-xs font-normal text-content2">"Kalibrasi Peralatan"</span>
            </div>
        </section>
    }
}
