use yew::prelude::*;
use crate::models::CandidateProfile;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct FormulirProps {
    pub profile: CandidateProfile,
}

#[function_component(Formulir)]
pub fn formulir(props: &FormulirProps) -> Html {
    let profile = &props.profile;
    let is_saved = use_state(|| false);

    let on_save_draft = {
        let is_saved = is_saved.clone();
        Callback::from(move |_| {
            is_saved.set(true);
            let is_saved_reset = is_saved.clone();
            gloo_timers::callback::Timeout::new(2000, move || {
                is_saved_reset.set(false);
            }).forget();
        })
    };

    let on_print = Callback::from(|_| {
        if let Some(win) = window() {
            let _ = win.print();
        }
    });

    html! {
        <div class="flex flex-col w-full gap-6">
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 p-6 rounded-2xl bg-white border border-slate-200 shadow-sm">
                <div class="flex flex-col gap-1">
                    <div class="flex items-center gap-2 text-xs text-[#006a61] font-bold uppercase tracking-wider">
                        <span class="material-symbols-outlined text-[18px]">{"assignment"}</span>
                        <span>{"Formulir Pendaftaran Calon Peserta PPDS"}</span>
                    </div>
                    <h1 class="text-xl sm:text-2xl font-extrabold text-[#001428]">
                        {"Biodata & Rekam Pengalaman Klinis Dokter"}
                    </h1>
                    <p class="text-xs text-slate-500">
                        {"Data resmi yang disinkronkan dengan Konsil Kedokteran Indonesia (KKI) & IDI."}
                    </p>
                </div>

                <div class="flex items-center gap-3">
                    <button 
                        type="button" 
                        onclick={on_save_draft} 
                        class="px-4 py-2.5 rounded-xl border border-slate-200 text-slate-700 hover:bg-slate-50 font-bold text-xs flex items-center gap-2 transition-all shadow-sm"
                    >
                        {
                            if *is_saved {
                                html! {
                                    <>
                                        <span class="material-symbols-outlined text-[18px] text-emerald-600">{"check_circle"}</span>
                                        <span class="text-emerald-700">{"Draf Tersimpan!"}</span>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        <span class="material-symbols-outlined text-[18px] text-slate-500">{"save"}</span>
                                        <span>{"Simpan Draf"}</span>
                                    </>
                                }
                            }
                        }
                    </button>
                    <button 
                        type="button" 
                        onclick={on_print} 
                        class="px-4 py-2.5 rounded-xl bg-[#0f2942] hover:bg-[#001428] text-white font-bold text-xs flex items-center gap-2 transition-all shadow-md shadow-slate-900/10"
                    >
                        <span class="material-symbols-outlined text-[18px]">{"print"}</span>
                        <span>{"Cetak Formulir"}</span>
                    </button>
                </div>
            </div>

            <section class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                <div class="flex items-center gap-2 pb-3 border-b border-slate-100">
                    <span class="flex items-center justify-center w-7 h-7 rounded-lg bg-[#006a61] text-white text-xs font-bold">{"1"}</span>
                    <h2 class="text-base font-bold text-[#001428]">{"Pilihan Program Studi & RS Pendidikan"}</h2>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Program Spesialis"}</span>
                        <span class="text-sm font-bold text-slate-800">{&profile.program_name}</span>
                        <span class="text-xs text-slate-500">{"Fakultas Kedokteran Universitas Tanjungpura"}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"RS Jejaring Utama Rotasi"}</span>
                        <span class="text-sm font-bold text-slate-800">{"RSUD Dokter Soedarso Pontianak"}</span>
                        <span class="text-xs text-slate-500">{"Rumah Sakit Pendidikan Utama Tipe A"}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Jalur Masuk Seleksi"}</span>
                        <span class="text-sm font-bold text-slate-800">{"Jalur Reguler Umum"}</span>
                        <span class="text-xs text-slate-500">{"Seleksi Kompetensi Terbuka Seluruh Indonesia"}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Skema Pembiayaan"}</span>
                        <span class="text-sm font-bold text-slate-800">{"Beasiswa LPDP Targeted Dokter Spesialis"}</span>
                        <span class="text-xs text-slate-500">{"Kementerian Keuangan & Kementerian Kesehatan RI"}</span>
                    </div>
                </div>
            </section>

            <section class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                <div class="flex items-center gap-2 pb-3 border-b border-slate-100">
                    <span class="flex items-center justify-center w-7 h-7 rounded-lg bg-[#006a61] text-white text-xs font-bold">{"2"}</span>
                    <h2 class="text-base font-bold text-[#001428]">{"Identitas Dokter & Keabsahan Profesi"}</h2>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Nama Lengkap Beserta Gelar"}</span>
                        <span class="text-sm font-bold text-slate-800">{&profile.full_name}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"NIK Kependudukan"}</span>
                        <span class="text-sm font-bold text-slate-800 font-mono">{&profile.nik}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Nomor STR Dokter KKI"}</span>
                        <span class="text-sm font-bold text-slate-800 font-mono">{&profile.str_number}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Masa Berlaku STR"}</span>
                        <span class="text-sm font-bold text-slate-800">{&profile.str_valid_until}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Nomor Pokok Anggota IDI"}</span>
                        <span class="text-sm font-bold text-slate-800 font-mono">{&profile.idi_number}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Almamater FK (S.Ked & dr.)"}</span>
                        <span class="text-sm font-bold text-slate-800">{"Universitas Airlangga (FK UNAIR)"}</span>
                    </div>
                </div>
            </section>

            <section class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                <div class="flex items-center gap-2 pb-3 border-b border-slate-100">
                    <span class="flex items-center justify-center w-7 h-7 rounded-lg bg-[#006a61] text-white text-xs font-bold">{"3"}</span>
                    <h2 class="text-base font-bold text-[#001428]">{"Pengalaman Klinis & Sertifikasi Kegawatdaruratan"}</h2>
                </div>

                <div class="flex flex-col gap-3">
                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                        <div class="flex items-start gap-3">
                            <span class="material-symbols-outlined text-[22px] text-[#006a61] mt-0.5">{"local_hospital"}</span>
                            <div>
                                <span class="text-xs sm:text-sm font-bold text-slate-800">
                                    {"Dokter Internsip (PIDI) — RSUD dr. Abdul Aziz Singkawang & Puskesmas Barat"}
                                </span>
                                <p class="text-xs text-slate-500 mt-0.5">
                                    {"Penugasan Kemenkes RI • Periode Februari 2023 - Februari 2024 (Lulus Sempurna)"}
                                </p>
                            </div>
                        </div>
                        <span class="px-2.5 py-1 rounded-full bg-emerald-50 text-emerald-700 text-xs font-bold self-start sm:self-auto">
                            {"Selesai (STSI Valid)"}
                        </span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                        <div class="flex items-start gap-3">
                            <span class="material-symbols-outlined text-[22px] text-[#006a61] mt-0.5">{"emergency"}</span>
                            <div>
                                <span class="text-xs sm:text-sm font-bold text-slate-800">
                                    {"Dokter Jaga IGD & Ruang Intensif — RS Santo Antonius Pontianak"}
                                </span>
                                <p class="text-xs text-slate-500 mt-0.5">
                                    {"Pelayanan Gawat Darurat Akut & Resusitasi • Maret 2024 - Sekarang"}
                                </p>
                            </div>
                        </div>
                        <span class="px-2.5 py-1 rounded-full bg-blue-50 text-blue-700 text-xs font-bold self-start sm:self-auto">
                            {"Aktif"}
                        </span>
                    </div>
                </div>

                <div class="pt-2">
                    <span class="text-xs font-bold text-slate-700 block mb-2">{"Sertifikasi Kompetensi Gawat Darurat Aktif:"}</span>
                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                        <div class="p-3 rounded-xl bg-slate-50 border border-slate-200 text-xs flex flex-col gap-1">
                            <span class="font-bold text-slate-800">{"ACLS (Advanced Cardiac Life Support)"}</span>
                            <span class="text-slate-500">{"PERKI Pusat • Berlaku s/d Nov 2027"}</span>
                        </div>
                        <div class="p-3 rounded-xl bg-slate-50 border border-slate-200 text-xs flex flex-col gap-1">
                            <span class="font-bold text-slate-800">{"ATLS (Advanced Trauma Life Support)"}</span>
                            <span class="text-slate-500">{"IKABI Pusat • Berlaku s/d Des 2027"}</span>
                        </div>
                        <div class="p-3 rounded-xl bg-slate-50 border border-slate-200 text-xs flex flex-col gap-1">
                            <span class="font-bold text-slate-800">{"FCCS (Fundamental Critical Care Support)"}</span>
                            <span class="text-slate-500">{"SCCM • Berlaku s/d Jan 2028"}</span>
                        </div>
                    </div>
                </div>
            </section>

            <section class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                <div class="flex items-center gap-2 pb-3 border-b border-slate-100">
                    <span class="flex items-center justify-center w-7 h-7 rounded-lg bg-[#006a61] text-white text-xs font-bold">{"4"}</span>
                    <h2 class="text-base font-bold text-[#001428]">{"Kontak Darurat & Integritas Hukum"}</h2>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Nama Kontak Darurat"}</span>
                        <span class="text-sm font-bold text-slate-800">{"dr. Nadia Putri, Sp.PK"}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Hubungan"}</span>
                        <span class="text-sm font-bold text-slate-800">{"Istri"}</span>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider">{"Nomor Telepon Darurat"}</span>
                        <span class="text-sm font-bold text-slate-800 font-mono">{"+62 812-9876-1122"}</span>
                    </div>
                </div>

                <div class="p-4 rounded-xl bg-teal-50/60 border border-[#006a61]/20 flex items-start gap-3">
                    <span class="material-symbols-outlined text-[22px] text-[#006a61] mt-0.5">{"gavel"}</span>
                    <div class="flex flex-col text-xs text-slate-700 leading-relaxed">
                        <span class="font-bold text-[#001428]">{"Pernyataan Hukum & Kepatuhan UU No. 17 Tahun 2023"}</span>
                        <p class="mt-0.5 text-slate-600">
                            {"Seluruh data identitas kependudukan, keanggotaan profesi, status STR KKI, serta rekam pengalaman klinis yang tercantum telah ditandatangani secara elektronik (e-Sign bersertifikasi BSrE) dan memiliki kekuatan hukum penuh."}
                        </p>
                    </div>
                </div>
            </section>
        </div>
    }
}
