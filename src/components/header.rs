use yew::prelude::*;
use yew_router::prelude::*;
use crate::models::CandidateProfile;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub profile: CandidateProfile,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let profile = &props.profile;
    let current_route = use_route::<Route>().unwrap_or(Route::Dashboard);
    let is_menu_open = use_state(|| false);

    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };

    let close_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(false);
        })
    };

    let nav_items = vec![
        (Route::Dashboard, "analytics", "Beranda & Status Seleksi"),
        (Route::Formulir, "assignment", "Formulir Pendaftaran"),
        (Route::UnggahBerkas, "upload_file", "Unggah Berkas & Foto"),
        (Route::Pengumuman, "verified", "Hasil & Pengumuman Seleksi"),
    ];

    html! {
        <>
            <header class="fixed top-0 left-0 right-0 bg-white/95 backdrop-blur-md border-b border-slate-200 z-50 shadow-xs">
                <div class="h-16 w-full px-4 sm:px-6 lg:px-8 flex items-center justify-between gap-2 sm:gap-4">
                    <div class="flex items-center gap-2.5 sm:gap-3.5 min-w-0">
                        <Link<Route> to={Route::Dashboard} classes="shrink-0 flex items-center">
                            <img 
                                src="https://lh3.googleusercontent.com/aida-public/AB6AXuDC4EVqWmwV0DgL_e_oEz63llQ0BTtmHNaJE8l3po1l6GCMT-Pkj9ZZ51V_Vc-KyO0bu_nfQVk_7zrH4rgz_-VpA-fiBLAuiPTn5u_XBhpGWeartm6Mhc_hJP7GAI8PeoDXnnKBWl2n8cowJa4xHTiQXziuNxWTtQ7WXIahNTUTBMgimvTX9w2pegxcifFVOt299yVczYmDeDNtVLmcWQtzStOLsu84U0zIFedKYzH79f-8h0xP-cmd" 
                                alt="Logo Untan" 
                                class="w-9 h-9 sm:w-10 sm:h-10 object-contain rounded-full shadow-xs border border-slate-100"
                            />
                        </Link<Route>>
                        <div class="flex flex-col min-w-0">
                            <span class="font-bold text-xs sm:text-sm lg:text-[15px] text-[#001428] tracking-tight truncate">
                                <span class="hidden sm:inline">{"PORTAL SELEKSI PPDS FAKULTAS KEDOKTERAN UNTAN"}</span>
                                <span class="inline sm:hidden">{"PORTAL PPDS FK UNTAN"}</span>
                            </span>
                            <span class="text-[10px] sm:text-xs text-slate-500 uppercase tracking-wider font-medium truncate">
                                {"UNIVERSITAS TANJUNGPURA & RS PENDIDIKAN KALBAR"}
                            </span>
                        </div>
                        <span class="hidden xl:inline-flex items-center px-2.5 py-1 rounded-full bg-teal-50 border border-teal-200/80 text-[#006a61] text-xs font-semibold ml-1.5 shrink-0">
                            <span class="w-1.5 h-1.5 rounded-full bg-[#006a61] mr-1.5 animate-pulse"></span>
                            {"Periode Ganjil 2025/2026"}
                        </span>
                    </div>

                    <div class="flex items-center gap-2 sm:gap-3 shrink-0">
                        <button 
                            aria-label="Pemberitahuan" 
                            class="relative p-2 rounded-xl text-slate-600 hover:text-slate-900 hover:bg-slate-100 transition-colors" 
                            type="button"
                        >
                            <span class="material-symbols-outlined text-[22px]">{"notifications"}</span>
                            <span class="absolute top-1.5 right-1.5 w-2 h-2 rounded-full bg-red-600 ring-2 ring-white"></span>
                        </button>

                        <div class="flex items-center gap-2.5 pl-2 sm:pl-3 border-l border-slate-200">
                            <img 
                                alt="Profile" 
                                class="w-8 h-8 sm:w-9 sm:h-9 rounded-full object-cover ring-2 ring-[#006a61]/30" 
                                src="https://lh3.googleusercontent.com/aida-public/AB6AXuB3iXfipsSIXgK7xUIK4nU8d_47ZekCMHCWjMonjVpa82n4E44GSPl3sseiCD_PRAv24auNSSHOMYKFPbrYGMmhF4ewg1ycxW4bpUlShdDS1DEpNix1Z1AoZQVDR7DHlfylnvjOM2ebMYkWAGoaPfKBpeVgHkk9xTFxbnFI0gMsklk17lXi_V-AZyHfFMiKvgSsI7UCep74_sXuVApxfvNE_A10yA0go4XPzwzV2Rt8CUv1JidvJLRM"
                            />
                            <div class="hidden sm:flex flex-col text-left">
                                <span class="text-xs sm:text-sm text-slate-900 font-bold leading-tight truncate max-w-[150px] lg:max-w-[190px]">
                                    {&profile.full_name}
                                </span>
                                <span class="text-[10px] sm:text-[11px] text-slate-500 font-mono">
                                    {format!("No. Reg: {}", &profile.registration_number)}
                                </span>
                            </div>
                        </div>

                        <button
                            type="button"
                            onclick={toggle_menu}
                            class="lg:hidden inline-flex items-center justify-center p-2 rounded-xl text-slate-700 hover:text-[#006a61] hover:bg-slate-100 border border-slate-200/80 transition-colors focus:outline-hidden"
                            aria-label="Toggle navigation menu"
                            aria-expanded={(*is_menu_open).to_string()}
                        >
                            <span class="material-symbols-outlined text-[24px]">
                                {if *is_menu_open { "close" } else { "menu" }}
                            </span>
                        </button>
                    </div>
                </div>

                <div class="hidden lg:flex h-12 w-full px-4 sm:px-6 lg:px-8 border-t border-slate-100 bg-slate-50/80 items-center justify-between">
                    <nav class="flex items-center gap-1.5" aria-label="Navigasi Utama">
                        {
                            for nav_items.iter().map(|(route, icon, label)| {
                                let is_active = current_route == *route;
                                let link_class = if is_active {
                                    "flex items-center gap-2 px-3.5 py-1.5 rounded-lg transition-all bg-[#006a61] text-white font-semibold text-xs sm:text-sm shadow-xs"
                                } else {
                                    "flex items-center gap-2 px-3.5 py-1.5 rounded-lg transition-colors text-slate-600 hover:bg-white hover:text-slate-900 font-medium text-xs sm:text-sm hover:shadow-2xs"
                                };

                                html! {
                                    <Link<Route> to={*route} classes={classes!(link_class)}>
                                        <span class="material-symbols-outlined text-[18px]">{*icon}</span>
                                        <span>{*label}</span>
                                    </Link<Route>>
                                }
                            })
                        }
                    </nav>

                    <div class="flex items-center gap-3">
                        <div class="flex items-center gap-1.5 text-xs text-slate-600 bg-white px-3 py-1 rounded-full border border-slate-200/80 shadow-2xs">
                            <span class="material-symbols-outlined text-[16px] text-[#006a61]">{"verified_user"}</span>
                            <span class="font-medium">{"Akun Terverifikasi (NPA IDI & STR KKI)"}</span>
                        </div>
                    </div>
                </div>
            </header>

            if *is_menu_open {
                <>
                    <div 
                        class="fixed inset-0 top-16 bg-slate-900/40 backdrop-blur-xs z-40 lg:hidden transition-opacity"
                        onclick={close_menu.clone()}
                        aria-hidden="true"
                    />

                    <div class="fixed top-16 left-0 right-0 max-h-[calc(100vh-4rem)] overflow-y-auto bg-white border-b border-slate-200 shadow-2xl z-50 px-4 sm:px-6 py-5 flex flex-col gap-4 lg:hidden animate-in slide-in-from-top-2 duration-150">
                        <div class="p-3.5 rounded-xl bg-slate-50 border border-slate-200/80 flex items-center gap-3">
                            <img 
                                alt="Profile" 
                                class="w-12 h-12 rounded-full object-cover ring-2 ring-[#006a61]/30 shrink-0" 
                                src="https://lh3.googleusercontent.com/aida-public/AB6AXuB3iXfipsSIXgK7xUIK4nU8d_47ZekCMHCWjMonjVpa82n4E44GSPl3sseiCD_PRAv24auNSSHOMYKFPbrYGMmhF4ewg1ycxW4bpUlShdDS1DEpNix1Z1AoZQVDR7DHlfylnvjOM2ebMYkWAGoaPfKBpeVgHkk9xTFxbnFI0gMsklk17lXi_V-AZyHfFMiKvgSsI7UCep74_sXuVApxfvNE_A10yA0go4XPzwzV2Rt8CUv1JidvJLRM"
                            />
                            <div class="flex flex-col min-w-0">
                                <span class="font-bold text-sm text-slate-900 truncate">
                                    {&profile.full_name}
                                </span>
                                <span class="text-xs text-slate-500 font-mono">
                                    {format!("No. Reg: {}", &profile.registration_number)}
                                </span>
                                <span class="text-[11px] text-[#006a61] font-medium flex items-center gap-1 mt-0.5">
                                    <span class="material-symbols-outlined text-[14px]">{"verified_user"}</span>
                                    {"NPA IDI Aktif • STR KKI Valid"}
                                </span>
                            </div>
                        </div>

                        <div class="flex flex-col gap-1">
                            <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider px-2 py-1">
                                {"Menu Navigasi Seleksi"}
                            </span>
                            <nav class="flex flex-col gap-1.5" aria-label="Navigasi Mobile">
                                {
                                    for nav_items.into_iter().map(|(route, icon, label)| {
                                        let is_active = current_route == route;
                                        let link_class = if is_active {
                                            "flex items-center justify-between px-4 py-3 rounded-xl transition-all bg-[#006a61] text-white font-semibold shadow-xs w-full"
                                        } else {
                                            "flex items-center justify-between px-4 py-3 rounded-xl transition-colors text-slate-700 hover:bg-slate-100 font-medium w-full"
                                        };

                                        let on_click = close_menu.clone();

                                        html! {
                                            <div onclick={on_click} class="w-full">
                                                <Link<Route> to={route} classes={classes!(link_class)}>
                                                    <div class="flex items-center gap-3">
                                                        <span class="material-symbols-outlined text-[22px]">{icon}</span>
                                                        <span class="text-sm">{label}</span>
                                                    </div>
                                                    <span class="material-symbols-outlined text-[18px] opacity-70">
                                                        {if is_active { "check_circle" } else { "chevron_right" }}
                                                    </span>
                                                </Link<Route>>
                                            </div>
                                        }
                                    })
                                }
                            </nav>
                        </div>

                        <div class="pt-2 border-t border-slate-100 flex flex-col gap-2">
                            <div class="p-3 rounded-xl bg-teal-50/60 border border-teal-100 text-[#006a61] flex items-center justify-between text-xs font-medium">
                                <div class="flex items-center gap-1.5">
                                    <span class="material-symbols-outlined text-[18px]">{"calendar_month"}</span>
                                    <span>{"Periode Ganjil 2025/2026"}</span>
                                </div>
                                <span class="bg-[#006a61] text-white text-[10px] font-bold px-2 py-0.5 rounded-full">
                                    {"Tahap 5"}
                                </span>
                            </div>
                            <p class="text-[11px] text-slate-400 text-center">
                                {"Sekretariat PPDS FK Universitas Tanjungpura & Konsorsium RS Pendidikan Kalbar"}
                            </p>
                        </div>
                    </div>
                </>
            }
        </>
    }
}
