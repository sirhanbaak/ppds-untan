use yew::prelude::*;
use crate::models::CandidateProfile;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub profile: CandidateProfile,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let profile = &props.profile;

    html! {
        <header class="fixed top-0 left-0 right-0 h-16 bg-white/90 backdrop-blur-xl border-b border-slate-200 shadow-[0_1px_8px_rgba(0,0,0,0.04)] z-50">
            <div class="h-16 w-full px-4 sm:px-6 flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <img 
                        src="https://lh3.googleusercontent.com/aida-public/AB6AXuDC4EVqWmwV0DgL_e_oEz63llQ0BTtmHNaJE8l3po1l6GCMT-Pkj9ZZ51V_Vc-KyO0bu_nfQVk_7zrH4rgz_-VpA-fiBLAuiPTn5u_XBhpGWeartm6Mhc_hJP7GAI8PeoDXnnKBWl2n8cowJa4xHTiQXziuNxWTtQ7WXIahNTUTBMgimvTX9w2pegxcifFVOt299yVczYmDeDNtVLmcWQtzStOLsu84U0zIFedKYzH79f-8h0xP-cmd" 
                        alt="Logo Untan" 
                        class="w-10 h-10 object-contain rounded-full shadow-sm"
                    />
                    <div class="flex flex-col">
                        <span class="font-bold text-xs sm:text-sm text-[#001428] tracking-tight">
                            {"PORTAL SELEKSI PPDS FAKULTAS KEDOKTERAN UNIVERSITAS TANJUNGPURA"}
                        </span>
                        <span class="text-[10px] sm:text-xs text-slate-500 uppercase tracking-wider font-medium">
                            {"UNIVERSITAS TANJUNGPURA (UNTAN) & KONSORSIUM RS PENDIDIKAN KALBAR"}
                        </span>
                    </div>
                    <span class="hidden xl:inline-flex items-center px-2.5 py-1 rounded-full bg-slate-100 text-slate-700 text-xs font-semibold ml-2">
                        <span class="w-1.5 h-1.5 rounded-full bg-[#006a61] mr-1.5"></span>
                        {"Periode Ganjil 2025/2026"}
                    </span>
                </div>

                <div class="flex items-center gap-4">
                    <button 
                        aria-label="Pemberitahuan" 
                        class="relative p-2 rounded-lg text-slate-600 hover:text-slate-900 hover:bg-slate-100 transition-colors" 
                        type="button"
                    >
                        <span class="material-symbols-outlined text-[22px]">{"notifications"}</span>
                        <span class="absolute top-1.5 right-1.5 w-2 h-2 rounded-full bg-red-600"></span>
                    </button>
                    <div class="flex items-center gap-3 pl-3 border-l border-slate-200">
                        <img 
                            alt="Profile" 
                            class="w-9 h-9 rounded-full object-cover ring-2 ring-[#006a61]/30" 
                            src="https://lh3.googleusercontent.com/aida-public/AB6AXuB3iXfipsSIXgK7xUIK4nU8d_47ZekCMHCWjMonjVpa82n4E44GSPl3sseiCD_PRAv24auNSSHOMYKFPbrYGMmhF4ewg1ycxW4bpUlShdDS1DEpNix1Z1AoZQVDR7DHlfylnvjOM2ebMYkWAGoaPfKBpeVgHkk9xTFxbnFI0gMsklk17lXi_V-AZyHfFMiKvgSsI7UCep74_sXuVApxfvNE_A10yA0go4XPzwzV2Rt8CUv1JidvJLRM"
                        />
                        <div class="hidden sm:flex flex-col text-left">
                            <span class="text-xs sm:text-sm text-slate-900 font-bold leading-tight">
                                {&profile.full_name}
                            </span>
                            <span class="text-[11px] text-slate-500 font-mono">
                                {format!("No. Reg: {}", &profile.registration_number)}
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}
