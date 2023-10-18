use leptos::*;

#[component]
pub fn DefaultSidebar() -> impl IntoView {
    view! {
        <ul class="mt-6 space-y-1">
            <li>
                <a href="" class="block rounded-lg bg-gray-100 px-4 py-2 text-sm font-medium text-gray-700">
                    General
                </a>
            </li>

            <li>
                <a href="" class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700">
                    Billing
                </a>
            </li>

            <li>
                <a href="" class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700">
                    Invoices
                </a>
            </li>
        </ul>
    }
}
