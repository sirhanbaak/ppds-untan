use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let current_route = use_route::<Route>().unwrap_or(Route::Dashboard);

    let nav_items = vec![
        (Route::Dashboard, "analytics", "Beranda & Status Seleksi"),
        (Route::Formulir, "assignment", "Formulir Pendaftaran"),
        (Route::UnggahBerkas, "upload_file", "Unggah Berkas & Foto"),
        (Route::Pengumuman, "verified", "Hasil & Pengumuman Seleksi"),
    ];

    html! {
        <aside class="fixed left-0 top-16 h-[calc(100vh-4rem)] w-72 bg-white border-r border-slate-200 shadow-[0_1px_8px_rgba(0,0,0,0.04)] z-40 flex flex-col justify-between py-6">
            <div class="px-4">
                <div class="px-3 mb-4">
                    <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">
                        {"Navigasi Seleksi"}
                    </span>
                </div>
                <nav class="flex flex-col gap-1.5">
                    {
                        for nav_items.into_iter().map(|(route, icon, label)| {
                            let is_active = current_route == route;
                            let link_class = if is_active {
                                "flex items-center gap-3 px-3 py-2.5 rounded-xl transition-all bg-[#0f2942] text-white font-semibold shadow-md shadow-slate-900/10"
                            } else {
                                "flex items-center gap-3 px-3 py-2.5 rounded-xl text-slate-600 hover:bg-slate-100 hover:text-slate-900 transition-colors text-sm font-medium"
                            };

                            html! {
                                <Link<Route> to={route} classes={classes!(link_class)}>
                                    <span class="material-symbols-outlined text-[20px]">{icon}</span>
                                    <span class="text-xs sm:text-sm">{label}</span>
                                </Link<Route>>
                            }
                        })
                    }
                </nav>
            </div>

            <div class="px-4">
                <div class="p-3.5 rounded-xl bg-slate-50 border border-slate-200/80 flex flex-col gap-1">
                    <div class="flex items-center gap-1.5 text-[#006a61]">
                        <span class="material-symbols-outlined text-[18px]">{"verified_user"}</span>
                        <span class="text-[11px] font-bold uppercase tracking-wider">{"Status Akun Terverifikasi"}</span>
                    </div>
                    <p class="text-xs text-slate-500 leading-relaxed">
                        {"NPA IDI Aktif & STR Dokter Umum teregistrasi KKI SatuSehat."}
                    </p>
                </div>
            </div>
        </aside>
    }
}
