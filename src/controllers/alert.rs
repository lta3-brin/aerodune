use leptos::*;

pub fn default_fallback(errors: RwSignal<Errors>) -> impl IntoView {
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
}