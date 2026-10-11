use yew::prelude::*;
use crate::models::CandidateProfile;
use crate::api::ApiService;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct PengumumanProps {
    pub profile: CandidateProfile,
}

#[function_component(Pengumuman)]
pub fn pengumuman(props: &PengumumanProps) -> Html {
    let profile = &props.profile;
    let interview = ApiService::get_interview_schedule();

    let on_print_decree = Callback::from(|_| {
        if let Some(win) = window() {
            let _ = win.print();
        }
    });

    let on_download_sk = Callback::from(|_| {
        gloo_dialogs::alert("Mengunduh Salinan Resmi Surat Keputusan Dekan FK UNTAN (SK No: 892/SK-P2/UNTAN/2025) berstempel QR IDI & BSrE...");
    });

    html! {
        <div class="flex flex-col w-full gap-6">
            <div class="flex flex-col lg:flex-row lg:items-end justify-between gap-4 p-6 rounded-2xl bg-white border border-slate-200 shadow-sm">
                <div class="flex flex-col gap-1">
                    <div class="flex items-center gap-2 text-xs text-[#006a61] font-bold uppercase tracking-wider">
                        <span class="material-symbols-outlined text-[18px]">{"verified"}</span>
                        <span>{"Pengumuman Terverifikasi Panitia Pusat Seleksi PPDS"}</span>
                    </div>
                    <h1 class="text-xl sm:text-2xl font-extrabold text-[#001428]">
                        {"Hasil Yudisium Tahap 2 & Transkrip Evaluasi Resmi"}
                    </h1>
                    <p class="text-xs text-slate-500">
                        {"Hasil evaluasi yudisium berkala Program Pendidikan Dokter Spesialis Periode Ganjil 2025/2026."}
                    </p>
                </div>

                <div class="flex items-center gap-2 bg-slate-50 px-4 py-2.5 rounded-xl border border-slate-200 text-slate-700 text-xs font-semibold">
                    <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"calendar_today"}</span>
                    <span>{"Diperbarui: "}<strong class="text-slate-900 font-bold">{"12 Agustus 2025, 14:00 WIB"}</strong></span>
                </div>
            </div>

            <div class="relative overflow-hidden rounded-2xl bg-[#0f2942] text-white p-6 sm:p-8 shadow-xl">
                <div class="relative z-10 flex flex-col gap-4">
                    <div class="inline-flex items-center gap-2 self-start px-3.5 py-1 rounded-full bg-[#006a61] text-white text-xs font-bold shadow-sm">
                        <span class="w-2 h-2 rounded-full bg-white animate-ping mr-1"></span>
                        {"KEPUTUSAN YUDISIUM RESMI TAHAP 2"}
                    </div>

                    <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-6">
                        <div class="max-w-3xl flex flex-col gap-2">
                            <h2 class="text-xl sm:text-2xl font-extrabold leading-snug">
                                {"SELAMAT! ANDA DINYATAKAN LULUS TAHAP 2 DAN BERHAK MENGIKUTI SELEKSI TAHAP 3 (WAWANCARA)"}
                            </h2>
                            <div class="flex flex-wrap items-center gap-x-4 gap-y-2 text-xs sm:text-sm text-slate-300">
                                <span class="font-bold text-teal-300">{&profile.program_name}</span>
                                <span class="text-slate-500">{"•"}</span>
                                <span class="font-mono">{&profile.registration_number}</span>
                                <span class="text-slate-500">{"•"}</span>
                                <span class="font-bold text-white">{&profile.full_name}</span>
                                <span class="text-slate-500">{"•"}</span>
                                <span class="px-2 py-0.5 rounded bg-teal-500/20 text-teal-300 font-bold">{"Peringkat 4 dari 86 Peserta"}</span>
                            </div>
                        </div>

                        <div class="flex flex-row lg:flex-col gap-3 shrink-0">
                            <button 
                                type="button" 
                                onclick={on_download_sk}
                                class="px-5 py-2.5 rounded-xl bg-[#006a61] hover:bg-[#005049] text-white text-xs font-bold flex items-center justify-center gap-2 shadow-md transition-all"
                            >
                                <span class="material-symbols-outlined text-[18px]">{"download"}</span>
                                <span>{"Unduh SK Kelulusan"}</span>
                            </button>
                            <button 
                                type="button" 
                                onclick={on_print_decree.clone()}
                                class="px-5 py-2.5 rounded-xl bg-white/10 hover:bg-white/20 text-white text-xs font-bold flex items-center justify-center gap-2 backdrop-blur-sm transition-all"
                            >
                                <span class="material-symbols-outlined text-[18px]">{"print"}</span>
                                <span>{"Cetak Bukti Lulus"}</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                    <span class="text-xs font-bold text-slate-400 uppercase tracking-wider">
                        {"Alur Tahapan Evaluasi Seleksi PPDS UNTAN"}
                    </span>
                    <span class="text-xs font-bold text-[#006a61]">
                        {"Progres: 75% Lengkap"}
                    </span>
                </div>

                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col justify-between">
                        <div class="flex items-center justify-between">
                            <div class="w-7 h-7 rounded-full bg-[#006a61] text-white flex items-center justify-center text-xs">
                                <span class="material-symbols-outlined text-[14px]">{"check"}</span>
                            </div>
                            <span class="px-2 py-0.5 rounded bg-slate-200 text-slate-700 text-[10px] font-bold">{"SELESAI"}</span>
                        </div>
                        <div class="mt-3">
                            <span class="text-xs font-bold text-slate-800">{"Tahap 1: Administrasi"}</span>
                            <p class="text-[11px] text-slate-500 mt-0.5">{"Verifikasi Ijazah, NPA IDI, & STR KKI"}</p>
                            <span class="text-xs font-bold text-emerald-600 block mt-2">{"Nilai: 94.50 (Lulus)"}</span>
                        </div>
                    </div>

                    <div class="p-4 rounded-xl bg-teal-50/50 border-2 border-[#006a61] flex flex-col justify-between shadow-sm">
                        <div class="flex items-center justify-between">
                            <div class="w-7 h-7 rounded-full bg-[#006a61] text-white flex items-center justify-center text-xs">
                                <span class="material-symbols-outlined text-[14px]">{"check"}</span>
                            </div>
                            <span class="px-2 py-0.5 rounded bg-[#006a61] text-white text-[10px] font-bold">{"TERKINI - LULUS"}</span>
                        </div>
                        <div class="mt-3">
                            <span class="text-xs font-bold text-[#001428]">{"Tahap 2: CBT & Kesehatan"}</span>
                            <p class="text-[11px] text-slate-500 mt-0.5">{"TPA, Ujian Tulis, MMPI-2 & MCU"}</p>
                            <span class="text-xs font-bold text-[#006a61] block mt-2">{"Skor TPA 592.50 • CBT 84.20"}</span>
                        </div>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col justify-between">
                        <div class="flex items-center justify-between">
                            <div class="w-7 h-7 rounded-full bg-[#0f2942] text-white flex items-center justify-center text-xs font-bold">
                                {"3"}
                            </div>
                            <span class="px-2 py-0.5 rounded bg-amber-100 text-amber-800 text-[10px] font-bold">{"SELANJUTNYA"}</span>
                        </div>
                        <div class="mt-3">
                            <span class="text-xs font-bold text-slate-800">{"Tahap 3: Wawancara"}</span>
                            <p class="text-[11px] text-slate-500 mt-0.5">{"Wawancara Dewan Panelis & Kasus"}</p>
                            <span class="text-xs font-bold text-amber-700 block mt-2">{"Jadwal: 18 Agustus 2025"}</span>
                        </div>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex flex-col justify-between opacity-60">
                        <div class="flex items-center justify-between">
                            <div class="w-7 h-7 rounded-full bg-slate-200 text-slate-500 flex items-center justify-center text-xs font-bold">
                                {"4"}
                            </div>
                            <span class="px-2 py-0.5 rounded bg-slate-200 text-slate-600 text-[10px] font-bold">{"MENUNGGU"}</span>
                        </div>
                        <div class="mt-3">
                            <span class="text-xs font-bold text-slate-600">{"Tahap 4: Yudisium Akhir"}</span>
                            <p class="text-[11px] text-slate-400 mt-0.5">{"Penetapan SK Rektor Untan"}</p>
                            <span class="text-xs font-medium text-slate-400 block mt-2">{"Target: 28 Agustus 2025"}</span>
                        </div>
                    </div>
                </div>
            </div>

            <section class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                    <div>
                        <h3 class="text-base font-bold text-[#001428]">{"Rekapitulasi Komponen Penilaian Yudisium"}</h3>
                        <p class="text-xs text-slate-500">{"Transkrip nilai terverifikasi Panitia Bersama FK UNTAN & Kolegium."}</p>
                    </div>
                </div>

                <div class="overflow-x-auto">
                    <table class="w-full text-left text-xs">
                        <thead>
                            <tr class="border-b border-slate-200 text-slate-400 font-bold uppercase tracking-wider text-[10px]">
                                <th class="pb-3 px-3">{"Komponen Evaluasi"}</th>
                                <th class="pb-3 px-3">{"Bobot"}</th>
                                <th class="pb-3 px-3">{"Ambang Batas"}</th>
                                <th class="pb-3 px-3">{"Nilai Peserta"}</th>
                                <th class="pb-3 px-3">{"Status Hasil"}</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-100">
                            <tr class="hover:bg-slate-50">
                                <td class="py-3 px-3 font-semibold text-slate-800">{"Kelengkapan Berkas Administrasi & Portofolio"}</td>
                                <td class="py-3 px-3 text-slate-500">{"15%"}</td>
                                <td class="py-3 px-3 text-slate-500">{"Lengkap (100%)"}</td>
                                <td class="py-3 px-3 font-mono font-bold text-slate-900">{"94.50"}</td>
                                <td class="py-3 px-3"><span class="px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700 font-bold text-[11px]">{"Lulus"}</span></td>
                            </tr>
                            <tr class="hover:bg-slate-50">
                                <td class="py-3 px-3 font-semibold text-slate-800">{"Tes Potensi Akademik (TPA Bappenas)"}</td>
                                <td class="py-3 px-3 text-slate-500">{"20%"}</td>
                                <td class="py-3 px-3 text-slate-500">{">= 500.00"}</td>
                                <td class="py-3 px-3 font-mono font-bold text-slate-900">{"592.50"}</td>
                                <td class="py-3 px-3"><span class="px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700 font-bold text-[11px]">{"Lulus"}</span></td>
                            </tr>
                            <tr class="hover:bg-slate-50">
                                <td class="py-3 px-3 font-semibold text-slate-800">{"CBT Pengetahuan Dasar Kedokteran Spesialis"}</td>
                                <td class="py-3 px-3 text-slate-500">{"30%"}</td>
                                <td class="py-3 px-3 text-slate-500">{">= 75.00"}</td>
                                <td class="py-3 px-3 font-mono font-bold text-slate-900">{"84.20"}</td>
                                <td class="py-3 px-3"><span class="px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700 font-bold text-[11px]">{"Lulus (Peringkat 4)"}</span></td>
                            </tr>
                            <tr class="hover:bg-slate-50">
                                <td class="py-3 px-3 font-semibold text-slate-800">{"Tes MMPI-2 & Pemeriksaan Psikometri"}</td>
                                <td class="py-3 px-3 text-slate-500">{"Syarat"}</td>
                                <td class="py-3 px-3 text-slate-500">{"Profil Valid"}</td>
                                <td class="py-3 px-3 font-bold text-slate-900">{"Stabil / Tidak Ada Deviasi"}</td>
                                <td class="py-3 px-3"><span class="px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700 font-bold text-[11px]">{"Rekomendasi"}</span></td>
                            </tr>
                            <tr class="hover:bg-slate-50">
                                <td class="py-3 px-3 font-semibold text-slate-800">{"Pemeriksaan Kesehatan Fisik (MCU) & Narkoba"}</td>
                                <td class="py-3 px-3 text-slate-500">{"Syarat"}</td>
                                <td class="py-3 px-3 text-slate-500">{"Fit to Practice"}</td>
                                <td class="py-3 px-3 font-bold text-slate-900">{"Sehat & Bebas Narkoba 6 Parameter"}</td>
                                <td class="py-3 px-3"><span class="px-2 py-0.5 rounded-full bg-emerald-50 text-emerald-700 font-bold text-[11px]">{"Lulus Medis"}</span></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </section>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                    <div class="flex items-center gap-2 pb-3 border-b border-slate-100">
                        <span class="material-symbols-outlined text-[#006a61] text-[22px]">{"event"}</span>
                        <h3 class="text-base font-bold text-[#001428]">{"Detail Sesi Wawancara (Tahap 3)"}</h3>
                    </div>

                    <div class="flex flex-col gap-3 text-xs">
                        <div class="flex items-center justify-between p-3 rounded-xl bg-slate-50 border border-slate-200">
                            <span class="text-slate-500">{"Hari & Tanggal:"}</span>
                            <span class="font-bold text-slate-800">{&interview.date_formatted}</span>
                        </div>
                        <div class="flex items-center justify-between p-3 rounded-xl bg-slate-50 border border-slate-200">
                            <span class="text-slate-500">{"Waktu Pelaksanaan:"}</span>
                            <span class="font-bold text-slate-800">{&interview.time_formatted}</span>
                        </div>
                        <div class="flex items-center justify-between p-3 rounded-xl bg-slate-50 border border-slate-200">
                            <span class="text-slate-500">{"Ruangan:"}</span>
                            <span class="font-bold text-slate-800">{&interview.room}</span>
                        </div>
                        <div class="p-3 rounded-xl bg-slate-50 border border-slate-200 text-slate-600">
                            {&interview.location}
                        </div>
                    </div>
                </div>

                <div class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                    <div class="flex items-center gap-2 pb-3 border-b border-slate-100">
                        <span class="material-symbols-outlined text-[#006a61] text-[22px]">{"checklist"}</span>
                        <h3 class="text-base font-bold text-[#001428]">{"Ketentuan Berkas Fisik Wawancara"}</h3>
                    </div>

                    <div class="flex flex-col gap-2.5 text-xs text-slate-600">
                        <div class="flex items-start gap-2.5">
                            <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"check_circle"}</span>
                            <span>{"Membawa Map Folio Biru rangkap 4 berisi seluruh salinan dokumen ijazah, STR, dan sertifikat."}</span>
                        </div>
                        <div class="flex items-start gap-2.5">
                            <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"check_circle"}</span>
                            <span>{"Membawa STR dan Ijazah asli untuk dicocokkan dengan fisik."}</span>
                        </div>
                        <div class="flex items-start gap-2.5">
                            <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"check_circle"}</span>
                            <span>{"Pakaian rapi: Kemeja formal berdasi, jas snelli dokter, bersepatu pantofel."}</span>
                        </div>
                        <div class="flex items-start gap-2.5">
                            <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"check_circle"}</span>
                            <span>{"Hadir minimal 30 menit sebelum jadwal sesi di Ruang Tunggu Dekanat FK UNTAN."}</span>
                        </div>
                    </div>
                </div>
            </div>

            <section class="p-6 sm:p-8 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-6">
                <div class="flex items-center justify-between pb-4 border-b border-slate-200">
                    <div class="flex items-center gap-3">
                        <img 
                            src="https://lh3.googleusercontent.com/aida-public/AB6AXuDC4EVqWmwV0DgL_e_oEz63llQ0BTtmHNaJE8l3po1l6GCMT-Pkj9ZZ51V_Vc-KyO0bu_nfQVk_7zrH4rgz_-VpA-fiBLAuiPTn5u_XBhpGWeartm6Mhc_hJP7GAI8PeoDXnnKBWl2n8cowJa4xHTiQXziuNxWTtQ7WXIahNTUTBMgimvTX9w2pegxcifFVOt299yVczYmDeDNtVLmcWQtzStOLsu84U0zIFedKYzH79f-8h0xP-cmd" 
                            alt="Logo Untan" 
                            class="w-12 h-12 object-contain"
                        />
                        <div class="flex flex-col">
                            <span class="text-xs font-bold text-slate-400 uppercase tracking-wider">{"Surat Keputusan Resmi"}</span>
                            <span class="text-sm sm:text-base font-extrabold text-[#001428]">{"FAKULTAS KEDOKTERAN UNIVERSITAS TANJUNGPURA"}</span>
                            <span class="text-xs text-slate-500">{"Nomor SK: 892/SK-P2/UNTAN/2025 • Tanggal Penetapan: 12 Agustus 2025"}</span>
                        </div>
                    </div>

                    <button 
                        type="button" 
                        onclick={on_print_decree}
                        class="px-4 py-2 rounded-xl border border-slate-200 text-slate-700 hover:bg-slate-50 font-bold text-xs flex items-center gap-1.5 transition-colors"
                    >
                        <span class="material-symbols-outlined text-[18px]">{"print"}</span>
                        <span>{"Cetak SK"}</span>
                    </button>
                </div>

                <div class="p-6 rounded-xl bg-slate-50 border border-slate-200 text-xs text-slate-700 leading-relaxed flex flex-col gap-3 font-serif">
                    <p class="font-bold text-center text-sm text-slate-900 font-sans">
                        {"TENTANG PENETAPAN KELULUSAN TAHAP II (UJIAN TULIS & KESEHATAN) CALON PESERTA PROGRAM PENDIDIKAN DOKTER SPESIALIS (PPDS) FAKULTAS KEDOKTERAN UNIVERSITAS TANJUNGPURA PERIODE GANJIL TAHUN AKADEMIK 2025/2026"}
                    </p>
                    <p class="mt-2">
                        {"Dekan Fakultas Kedokteran Universitas Tanjungpura, setelah memeriksa dan memverifikasi rekapitulasi nilai Ujian Kemampuan Dasar, Tes Potensi Akademik, serta kelayakan hasil pemeriksaan fisik dan psikometri dari Rumah Sakit Pendidikan Utama, dengan ini memutuskan bahwa:"}
                    </p>
                    <div class="p-4 rounded-lg bg-white border border-slate-200 font-sans text-xs flex flex-col gap-1.5 my-2">
                        <div class="flex"><span class="w-36 text-slate-400">{"Nama Lengkap"}</span><span class="font-bold text-slate-800">{format!(": {}", &profile.full_name)}</span></div>
                        <div class="flex"><span class="w-36 text-slate-400">{"Nomor Registrasi"}</span><span class="font-mono text-slate-800">{format!(": {}", &profile.registration_number)}</span></div>
                        <div class="flex"><span class="w-36 text-slate-400">{"Peminatan Spesialis"}</span><span class="font-bold text-[#006a61]">{format!(": {}", &profile.program_name)}</span></div>
                        <div class="flex"><span class="w-36 text-slate-400">{"Hasil Yudisium"}</span><span class="font-bold text-emerald-700">{": MEMENUHI SYARAT (LULUS) KE TAHAP III WAWANCARA"}</span></div>
                    </div>
                    <p>
                        {"Ditetapkan di Pontianak pada tanggal 12 Agustus 2025 oleh Dekan Fakultas Kedokteran Universitas Tanjungpura dan disahkan oleh Komite Koordinasi Pendidikan (KOMKORDIK)."}
                    </p>
                </div>
            </section>
        </div>
    }
}
