mod models;
mod api;
mod routes;
mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;
use routes::Route;
use components::Header;
use pages::{Dashboard, Formulir, Berkas, Pengumuman};
use api::ApiService;

fn switch(route: Route) -> Html {
    let profile = ApiService::get_candidate_profile();

    match route {
        Route::Dashboard => html! { <Dashboard profile={profile} /> },
        Route::Formulir => html! { <Formulir profile={profile} /> },
        Route::UnggahBerkas => html! { <Berkas profile={profile} /> },
        Route::Pengumuman => html! { <Pengumuman profile={profile} /> },
        Route::NotFound => html! { <Redirect<Route> to={Route::Dashboard} /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let profile = ApiService::get_candidate_profile();

    html! {
        <BrowserRouter>
            <div class="min-h-screen bg-[#f8f9ff] text-slate-800 font-sans antialiased selection:bg-[#006a61] selection:text-white flex flex-col">
                <Header profile={profile} />
                <div class="w-full flex-1 pt-20 lg:pt-32">
                    <main class="w-full px-4 sm:px-6 lg:px-8 py-6 sm:py-8 max-w-7xl mx-auto">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
                <footer class="w-full border-t border-slate-200/80 bg-white/70 py-5 text-center text-xs text-slate-500">
                    <div class="max-w-7xl mx-auto px-4 flex flex-col sm:flex-row items-center justify-between gap-2">
                        <span>{"© 2025-2026 Portal Seleksi PPDS Fakultas Kedokteran Universitas Tanjungpura"}</span>
                        <div class="flex items-center gap-4 text-slate-400">
                            <span>{"Kemenkes RI"}</span>
                            <span>{"•"}</span>
                            <span>{"Konsorsium RS Pendidikan Kalbar"}</span>
                            <span>{"•"}</span>
                            <span>{"KKI & SatuSehat"}</span>
                        </div>
                    </div>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
