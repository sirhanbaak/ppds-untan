mod models;
mod api;
mod routes;
mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;
use routes::Route;
use components::{Header, Sidebar};
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
            <div class="min-h-screen bg-[#f8f9ff] text-slate-800 font-sans antialiased selection:bg-[#006a61] selection:text-white">
                <Header profile={profile} />
                <Sidebar />
                <div class="pl-72 pt-16 min-h-[calc(100vh-4rem)]">
                    <main class="w-full px-6 sm:px-10 py-8 max-w-7xl mx-auto">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
