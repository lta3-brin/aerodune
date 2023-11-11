use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use anyhow::Result;
use surrealdb::engine::remote::ws::Ws;

use crate::components::alert::SuccessAlert;
use crate::components::galat::TakDitemukan;
use crate::components::{buttongroups::DefaultBtns, sidebar::DefaultSidebar};
use crate::pages::kegiatan::index::Kegiatan;
use crate::pages::kegiatan::rincian::RincianKegiatan;
use crate::pages::kegiatan::tambah::TambahKegiatan;
use crate::pages::utama::HalamanUtama;
use crate::stores::default::DefaultState;
use crate::configs::db::DB;

async fn connect_db() -> Result<()> {
    DB.connect::<Ws>("localhost:8000").await?;
    DB.use_ns("aerodune").use_db("kalibrasi").await?;

    Ok(())
}

#[component]
pub fn DefaultApp() -> impl IntoView {
    provide_meta_context();
    provide_context(create_rw_signal(DefaultState::default()));

    let act = create_action(|_: &()| {
        connect_db()
    });

    let state = expect_context::<RwSignal<DefaultState>>();
    let (light, _) = create_slice(state, |st| st.light, |st, val| st.light = val);
    let (side, _) = create_slice(state, |st| st.closesidebar, |st, val| st.closesidebar = val);
    let (success, _) = create_slice(state, |st| st.successalert, |st, val| st.successalert = val);

    act.dispatch(());
    view! {
        <Show when=light fallback=|| view! {
            <Html lang="en" class="dark bg-gray-800" />
        }>
            <Html lang="en" class="light" />
        </Show>

        <Router>
            <div class="p-7 lg:ml-60 text-gray-500 dark:text-gray-300">
                <div class="flex flex-row-reverse mb-3">
                    <DefaultBtns />
                </div>

                <Show
                    when=success fallback=|| view! {}>
                    <SuccessAlert />
                </Show>

                <Routes>
                    <Route path="/" view=HalamanUtama />
                    <Route path="/kegiatan" view=Kegiatan>
                        <Route path="" view=TambahKegiatan />
                        <Route path=":id" view=RincianKegiatan />
                    </Route>
                    <Route path="/*any" view=TakDitemukan />
                </Routes>
            </div>
        </Router>

        <div class="absolute top-0 left-0 invisible lg:visible">
            <DefaultSidebar />
        </div>

        <div class="absolute top-0 left-0 z-50 hidden"
            class:hidden=side
        >
            <DefaultSidebar />
        </div>
    }
}

