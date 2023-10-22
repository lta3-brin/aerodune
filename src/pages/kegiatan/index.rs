use leptos::*;
use leptos_router::*;

#[component]
pub fn Kegiatan() -> impl IntoView {
    view! {
        <div class="mt-4">
            <Outlet />
        </div>
    }
}

