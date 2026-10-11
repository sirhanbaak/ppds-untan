use yew::prelude::*;
use yew_router::prelude::*;
use crate::models::{CandidateProfile, StageStatus};
use crate::api::ApiService;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct DashboardProps {
    pub profile: CandidateProfile,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let profile = &props.profile;
    let stages = ApiService::get_selection_stages();
    let interview = ApiService::get_interview_schedule();
    let evaluations = ApiService::get_evaluations();
    let announcements = ApiService::get_announcements();

    html! {
        <div class="flex flex-col w-full gap-6">
            <section class="relative overflow-hidden rounded-2xl bg-white p-6 sm:p-8 shadow-sm border border-slate-200">
                <div class="absolute -right-16 -top-16 w-80 h-80 rounded-full bg-[#86f2e4]/15 blur-3xl pointer-events-none"></div>
                <div class="absolute right-1/4 -bottom-20 w-64 h-64 rounded-full bg-blue-100/30 blur-2xl pointer-events-none"></div>
                
                <div class="relative z-10 flex flex-col xl:flex-row xl:items-center justify-between gap-6">
                    <div class="flex flex-col gap-2 max-w-3xl">
                        <div class="flex flex-wrap items-center gap-2">
                            <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-slate-100 text-[11px] font-bold text-[#001428] uppercase tracking-wider">
                                <span class="w-2 h-2 rounded-full bg-[#006a61] animate-pulse"></span>
                                {"PORTAL SELEKSI PPDS UNIVERSITAS TANJUNGPURA TA 2025/2026"}
                            </span>
                            <span class="text-xs text-slate-500 font-mono bg-slate-50 px-2 py-0.5 rounded border border-slate-200">
                                {&profile.registration_number}
                            </span>
                        </div>

                        <h1 class="text-2xl sm:text-3xl font-extrabold text-[#001428] tracking-tight">
                            {format!("Selamat Datang, {}", &profile.full_name)}
                        </h1>

                        <div class="flex flex-wrap items-center gap-x-4 gap-y-1.5 text-slate-600 text-xs sm:text-sm">
                            <div class="flex items-center gap-1.5 text-slate-900 font-semibold">
                                <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"vital_signs"}</span>
                                <span>{&profile.program_name}</span>
                            </div>
                            <span class="text-slate-300">{"•"}</span>
                            <div class="flex items-center gap-1.5">
                                <span class="material-symbols-outlined text-[18px] text-slate-400">{"account_balance"}</span>
                                <span>{&profile.hospital_affiliation}</span>
                            </div>
                            <span class="text-slate-300">{"•"}</span>
                            <div class="flex items-center gap-1.5 text-[#006a61] font-semibold">
                                <span class="material-symbols-outlined text-[18px]">{"group"}</span>
                                <span>{format!("Daya Tampung: {} Kursi Residen (2 Orang)", profile.quota)}</span>
                            </div>
                        </div>
                    </div>

                    <div class="flex items-center gap-4 p-4 rounded-xl bg-gradient-to-br from-[#006a61]/10 via-slate-50 to-teal-50/40 border border-[#006a61]/30 shadow-sm shrink-0">
                        <div class="flex items-center justify-center w-12 h-12 rounded-xl bg-[#006a61] text-white shadow-md">
                            <span class="material-symbols-outlined text-[28px]">{"verified"}</span>
                        </div>
                        <div class="flex flex-col">
                            <span class="text-[11px] uppercase tracking-wider text-[#006a61] font-bold">
                                {"Status Utama Tahapan"}
                            </span>
                            <span class="text-sm sm:text-base font-bold text-[#001428]">
                                {&profile.overall_status}
                            </span>
                            <div class="flex items-center gap-1.5 mt-0.5 text-xs text-slate-500 font-medium">
                                <span class="w-2 h-2 rounded-full bg-[#006a61]"></span>
                                <span>{"Menunggu Pelaksanaan Tahap 3 (Wawancara Terjadwal)"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <section class="flex flex-col gap-4 rounded-2xl bg-white p-6 shadow-sm border border-slate-200">
                <div class="flex flex-col sm:flex-row sm:items-center justify-between pb-3 border-b border-slate-100 gap-2">
                    <div>
                        <span class="text-xs text-[#006a61] uppercase font-bold tracking-wider">
                            {"Metodologi Seleksi Terpadu"}
                        </span>
                        <h2 class="text-lg font-bold text-[#001428]">
                            {"Pelacak Tahapan Seleksi Spesialis"}
                        </h2>
                    </div>
                    <div class="flex items-center gap-1.5 text-slate-600 text-xs bg-slate-100 px-3 py-1 rounded-full w-fit font-medium">
                        <span class="material-symbols-outlined text-[16px] text-[#006a61]">{"update"}</span>
                        <span>{"Sinkronisasi Otomatis Kemenkes & Kolegium"}</span>
                    </div>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-6 gap-4 pt-2">
                    {
                        for stages.iter().map(|stg| {
                            let (badge_bg, line_bg, icon, status_text) = match stg.status {
                                StageStatus::Completed => ("bg-[#006a61] text-white", "bg-[#006a61]", "check", "Selesai"),
                                StageStatus::Current => ("bg-[#0f2942] text-white ring-4 ring-[#86f2e4]/40 animate-pulse", "bg-slate-200", "schedule", "Jadwal Aktif"),
                                StageStatus::Upcoming => ("bg-slate-100 text-slate-400 border border-slate-300", "bg-slate-200", "hourglass_empty", "Menunggu"),
                            };

                            html! {
                                <div class="flex flex-col gap-2 p-3 rounded-xl hover:bg-slate-50 transition-colors">
                                    <div class="flex items-center gap-2">
                                        <div class={classes!("flex", "items-center", "justify-center", "w-8", "h-8", "rounded-full", "shadow-sm", "text-sm", badge_bg)}>
                                            <span class="material-symbols-outlined text-[16px]">{icon}</span>
                                        </div>
                                        <div class={classes!("h-1", "flex-1", "rounded-full", line_bg)}></div>
                                    </div>
                                    <div class="flex flex-col">
                                        <span class="text-[10px] font-bold uppercase tracking-wider text-[#006a61]">
                                            {format!("Tahap {} • {}", stg.stage_number, status_text)}
                                        </span>
                                        <span class="text-xs sm:text-sm font-bold text-slate-800 mt-0.5">
                                            {&stg.title}
                                        </span>
                                        <span class="text-[11px] text-slate-400 font-medium">
                                            {&stg.date}
                                        </span>
                                        <p class="text-xs text-slate-500 mt-1 line-clamp-2">
                                            {&stg.description}
                                        </p>
                                    </div>
                                </div>
                            }
                        })
                    }
                </div>
            </section>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2 flex flex-col gap-6">
                    <div class="rounded-2xl bg-white p-6 shadow-sm border border-slate-200 flex flex-col gap-4">
                        <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                            <div class="flex items-center gap-2">
                                <span class="material-symbols-outlined text-[#006a61] text-[24px]">{"event_seat"}</span>
                                <h3 class="text-base font-bold text-[#001428]">
                                    {"Jadwal Sesi Wawancara Mendatang (Tahap 3)"}
                                </h3>
                            </div>
                            <span class="px-2.5 py-1 rounded-full bg-teal-50 text-[#006a61] border border-[#006a61]/20 text-xs font-bold">
                                {"Terkonfirmasi"}
                            </span>
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div class="p-4 rounded-xl bg-slate-50 border border-slate-200/80 flex flex-col gap-1.5">
                                <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Waktu & Tanggal"}</span>
                                <div class="flex items-center gap-2 text-sm font-bold text-slate-800">
                                    <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"calendar_today"}</span>
                                    <span>{&interview.date_formatted}</span>
                                </div>
                                <div class="flex items-center gap-2 text-xs text-slate-600 font-medium">
                                    <span class="material-symbols-outlined text-[18px] text-slate-400">{"schedule"}</span>
                                    <span>{&interview.time_formatted}</span>
                                </div>
                            </div>

                            <div class="p-4 rounded-xl bg-slate-50 border border-slate-200/80 flex flex-col gap-1.5">
                                <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Lokasi Pelaksanaan"}</span>
                                <div class="flex items-center gap-2 text-sm font-bold text-slate-800">
                                    <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"location_on"}</span>
                                    <span>{&interview.room}</span>
                                </div>
                                <div class="text-xs text-slate-500">
                                    {&interview.location}
                                </div>
                            </div>
                        </div>

                        <div class="flex flex-col gap-2 pt-2">
                            <span class="text-xs font-bold text-slate-700">
                                {&interview.panel_title}
                            </span>
                            <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
                                {
                                    for interview.panel_members.iter().map(|member| {
                                        html! {
                                            <div class="p-2.5 rounded-lg bg-slate-50 border border-slate-200 text-xs text-slate-700 font-medium flex items-center gap-2">
                                                <span class="material-symbols-outlined text-[16px] text-[#006a61]">{"person"}</span>
                                                <span class="line-clamp-2">{member}</span>
                                            </div>
                                        }
                                    })
                                }
                            </div>
                        </div>
                    </div>

                    <div class="rounded-2xl bg-white p-6 shadow-sm border border-slate-200 flex flex-col gap-4">
                        <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                            <div class="flex items-center gap-2">
                                <span class="material-symbols-outlined text-[#006a61] text-[24px]">{"fact_check"}</span>
                                <h3 class="text-base font-bold text-[#001428]">
                                    {"Transkrip Nilai Tahap 1 & Tahap 2"}
                                </h3>
                            </div>
                            <Link<Route> to={Route::Pengumuman} classes="text-xs font-semibold text-[#006a61] hover:underline flex items-center gap-1">
                                <span>{"Lihat Rincian Yudisium"}</span>
                                <span class="material-symbols-outlined text-[14px]">{"arrow_forward"}</span>
                            </Link<Route>>
                        </div>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            {
                                for evaluations.iter().map(|ev| {
                                    let score_display = match ev.score_numeric {
                                        Some(num) => format!("{:.2}", num),
                                        None => "-".to_string(),
                                    };
                                    let qual_display = ev.score_qualitative.clone().unwrap_or_default();

                                    html! {
                                        <div class="p-4 rounded-xl border border-slate-200 hover:border-slate-300 transition-all bg-white flex flex-col justify-between">
                                            <div class="flex flex-col gap-1">
                                                <span class="text-[10px] font-bold uppercase tracking-wider text-slate-400">
                                                    {&ev.category}
                                                </span>
                                                <span class="text-xs sm:text-sm font-bold text-slate-800">
                                                    {&ev.component_name}
                                                </span>
                                            </div>
                                            <div class="flex items-end justify-between mt-4 pt-3 border-t border-slate-100">
                                                <div>
                                                    <span class="text-xl font-extrabold text-[#001428] font-mono">
                                                        {score_display}
                                                    </span>
                                                    <p class="text-[11px] text-slate-500 font-medium">
                                                        {qual_display}
                                                    </p>
                                                </div>
                                                <span class="px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700 border border-emerald-200 text-xs font-bold">
                                                    {&ev.status}
                                                </span>
                                            </div>
                                        </div>
                                    }
                                })
                            }
                        </div>
                    </div>
                </div>

                <div class="flex flex-col gap-6">
                    <div class="rounded-2xl bg-white p-6 shadow-sm border border-slate-200 flex flex-col gap-4">
                        <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                            <div class="flex items-center gap-2">
                                <span class="material-symbols-outlined text-[#006a61] text-[22px]">{"folder_managed"}</span>
                                <h3 class="text-sm sm:text-base font-bold text-[#001428]">
                                    {"Integritas Berkas"}
                                </h3>
                            </div>
                            <span class="px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700 border border-emerald-200 text-xs font-bold">
                                {"9/9 Lengkap"}
                            </span>
                        </div>

                        <div class="flex items-center gap-3 p-3 rounded-xl bg-slate-50 border border-slate-200/80">
                            <span class="material-symbols-outlined text-emerald-600 text-[26px]">{"task_alt"}</span>
                            <div class="flex flex-col">
                                <span class="text-xs font-bold text-slate-800">{"Semua Dokumen Terverifikasi"}</span>
                                <span class="text-[11px] text-slate-500">{"TTE BSrE Kemenkes & SatuSehat Valid"}</span>
                            </div>
                        </div>

                        <div class="flex flex-col divide-y divide-slate-100 text-xs">
                            <div class="py-2 flex items-center justify-between">
                                <span class="text-slate-600">{"Ijazah & Transkrip S.Ked"}</span>
                                <span class="text-emerald-600 font-bold">{"Valid"}</span>
                            </div>
                            <div class="py-2 flex items-center justify-between">
                                <span class="text-slate-600">{"Ijazah & Transkrip Profesi dr."}</span>
                                <span class="text-emerald-600 font-bold">{"Valid"}</span>
                            </div>
                            <div class="py-2 flex items-center justify-between">
                                <span class="text-slate-600">{"STR Dokter Aktif (KKI)"}</span>
                                <span class="text-emerald-600 font-bold">{"Valid"}</span>
                            </div>
                            <div class="py-2 flex items-center justify-between">
                                <span class="text-slate-600">{"Sertifikat ACLS / ATLS"}</span>
                                <span class="text-emerald-600 font-bold">{"Valid"}</span>
                            </div>
                        </div>

                        <Link<Route> to={Route::UnggahBerkas} classes="mt-2 w-full py-2.5 rounded-xl bg-slate-100 hover:bg-slate-200 text-slate-800 text-center text-xs font-bold transition-colors flex items-center justify-center gap-1.5">
                            <span class="material-symbols-outlined text-[18px]">{"visibility"}</span>
                            <span>{"Kelola Berkas Lengkap"}</span>
                        </Link<Route>>
                    </div>

                    <div class="rounded-2xl bg-white p-6 shadow-sm border border-slate-200 flex flex-col gap-4">
                        <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                            <div class="flex items-center gap-2">
                                <span class="material-symbols-outlined text-[#006a61] text-[22px]">{"campaign"}</span>
                                <h3 class="text-sm sm:text-base font-bold text-[#001428]">
                                    {"Warta & Pengumuman"}
                                </h3>
                            </div>
                            <span class="text-[11px] text-slate-400 font-medium">{"Terbaru"}</span>
                        </div>

                        <div class="flex flex-col gap-3">
                            {
                                for announcements.iter().map(|ann| {
                                    html! {
                                        <div class="p-3 rounded-xl bg-slate-50 border border-slate-200/80 flex flex-col gap-1">
                                            <span class="text-xs font-bold text-slate-800">
                                                {&ann.title}
                                            </span>
                                            <p class="text-[11px] text-slate-500 leading-relaxed">
                                                {&ann.content}
                                            </p>
                                            <span class="text-[10px] text-slate-400 font-mono mt-1">
                                                {&ann.date}
                                            </span>
                                        </div>
                                    }
                                })
                            }
                        </div>
                    </div>

                    <div class="p-5 rounded-2xl bg-[#0f2942] text-white flex flex-col gap-2.5 shadow-md">
                        <div class="flex items-center gap-2 text-teal-300">
                            <span class="material-symbols-outlined text-[20px]">{"support_agent"}</span>
                            <span class="text-xs font-bold uppercase tracking-wider">{"Helpdesk Seleksi PPDS"}</span>
                        </div>
                        <p class="text-xs text-slate-300 leading-relaxed">
                            {"Sekretariat Panitia Seleksi PPDS Gedung FK UNTAN Pontianak, WhatsApp: 0812-5678-UNTAN"}
                        </p>
                        <div class="pt-2 border-t border-slate-700/60 flex items-center justify-between text-xs">
                            <span class="text-[10px] text-slate-400">{"Senin - Jumat 08:00 - 16:00"}</span>
                            <a href="https://wa.me/6281256788682" target="_blank" class="text-teal-300 font-bold hover:underline flex items-center gap-1">
                                <span>{"Hubungi WA"}</span>
                                <span class="material-symbols-outlined text-[14px]">{"chat"}</span>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
