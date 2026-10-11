use yew::prelude::*;
use crate::models::{CandidateProfile, DocumentItem};
use crate::api::ApiService;

#[derive(Properties, PartialEq)]
pub struct BerkasProps {
    pub profile: CandidateProfile,
}

#[function_component(Berkas)]
pub fn berkas(_props: &BerkasProps) -> Html {
    let documents = ApiService::get_documents();
    let selected_doc = use_state(|| Option::<DocumentItem>::None);
    let is_photo_modal_open = use_state(|| false);
    let is_locked = use_state(|| false);

    let on_preview_doc = {
        let selected_doc = selected_doc.clone();
        Callback::from(move |doc: DocumentItem| {
            selected_doc.set(Some(doc));
        })
    };

    let on_close_doc_modal = {
        let selected_doc = selected_doc.clone();
        Callback::from(move |_| {
            selected_doc.set(None);
        })
    };

    let on_open_photo_modal = {
        let is_photo_modal_open = is_photo_modal_open.clone();
        Callback::from(move |_| {
            is_photo_modal_open.set(true);
        })
    };

    let on_close_photo_modal = {
        let is_photo_modal_open = is_photo_modal_open.clone();
        Callback::from(move |_| {
            is_photo_modal_open.set(false);
        })
    };

    let on_lock_submission = {
        let is_locked = is_locked.clone();
        Callback::from(move |_| {
            if gloo_dialogs::confirm("Apakah Anda yakin ingin mengunci seluruh berkas seleksi PPDS? Setelah dikunci, perbaikan berkas memerlukan permohonan ke Sekretariat Kolegium.") {
                is_locked.set(true);
                gloo_dialogs::alert("Berkas seleksi berhasil dikunci dan ditandatangani secara digital. Nomor Berita Acara: BA-PPDS/2025/VII-0892.");
            }
        })
    };

    let on_reupload_click = Callback::from(|title: String| {
        gloo_dialogs::alert(&format!("Formulir unggah ulang untuk '{}' dibuka. Silakan pilih berkas PDF pengganti (maksimal 5MB).", title));
    });

    let on_download_zip = Callback::from(|_| {
        gloo_dialogs::alert("Mengunduh arsip kompresi resmi seluruh berkas persyaratan (Format ZIP, TTE BSrE Kemenkes tersemat)...");
    });

    html! {
        <div class="flex flex-col w-full gap-6">
            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 p-6 rounded-2xl bg-white border border-slate-200 shadow-sm">
                <div class="flex flex-col gap-1">
                    <div class="flex items-center gap-2 text-xs text-[#006a61] font-bold uppercase tracking-wider">
                        <span class="material-symbols-outlined text-[18px]">{"upload_file"}</span>
                        <span>{"Dokumen Persyaratan Seleksi PPDS"}</span>
                    </div>
                    <h1 class="text-xl sm:text-2xl font-extrabold text-[#001428]">
                        {"Unggah Berkas & Pasfoto Resmi Dokter"}
                    </h1>
                    <p class="text-xs text-slate-500">
                        {"Verifikasi digital dokumen persyaratan dokter spesialis FK UNTAN & Kemenkes RI."}
                    </p>
                </div>

                <div class="flex items-center gap-3">
                    <button 
                        type="button" 
                        onclick={on_download_zip}
                        class="px-4 py-2.5 rounded-xl border border-slate-200 text-slate-700 hover:bg-slate-50 font-bold text-xs flex items-center gap-2 transition-all shadow-sm"
                    >
                        <span class="material-symbols-outlined text-[18px]">{"folder_zip"}</span>
                        <span>{"Unduh Semua (ZIP)"}</span>
                    </button>
                    <button 
                        type="button" 
                        onclick={on_lock_submission}
                        disabled={*is_locked}
                        class={classes!(
                            "px-4", "py-2.5", "rounded-xl", "font-bold", "text-xs", "flex", "items-center", "gap-2", "transition-all", "shadow-md",
                            if *is_locked { "bg-emerald-600 text-white cursor-default" } else { "bg-[#0f2942] hover:bg-[#001428] text-white" }
                        )}
                    >
                        <span class="material-symbols-outlined text-[18px]">{if *is_locked { "lock" } else { "lock_open" }}</span>
                        <span>{if *is_locked { "Berkas Telah Dikunci" } else { "Kunci Pendaftaran" }}</span>
                    </button>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col items-center text-center gap-4">
                    <span class="text-xs font-bold uppercase tracking-wider text-slate-400 self-start">
                        {"Pasfoto Resmi Peserta"}
                    </span>
                    <div class="relative group cursor-pointer" onclick={on_open_photo_modal.clone()}>
                        <div class="w-36 h-48 rounded-xl overflow-hidden shadow-md ring-4 ring-slate-100 bg-[#b91c1c] flex items-center justify-center">
                            <img 
                                src="https://lh3.googleusercontent.com/aida-public/AB6AXuB3iXfipsSIXgK7xUIK4nU8d_47ZekCMHCWjMonjVpa82n4E44GSPl3sseiCD_PRAv24auNSSHOMYKFPbrYGMmhF4ewg1ycxW4bpUlShdDS1DEpNix1Z1AoZQVDR7DHlfylnvjOM2ebMYkWAGoaPfKBpeVgHkk9xTFxbnFI0gMsklk17lXi_V-AZyHfFMiKvgSsI7UCep74_sXuVApxfvNE_A10yA0go4XPzwzV2Rt8CUv1JidvJLRM" 
                                alt="Pasfoto Dokter" 
                                class="w-full h-full object-cover object-top"
                            />
                        </div>
                        <div class="absolute inset-0 bg-black/40 rounded-xl opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity text-white text-xs font-bold gap-1">
                            <span class="material-symbols-outlined text-[20px]">{"zoom_in"}</span>
                            <span>{"Pratinjau"}</span>
                        </div>
                    </div>

                    <div class="flex flex-col gap-1">
                        <span class="text-xs font-bold text-slate-800">{"Standar Pasfoto KKI (4x6)"}</span>
                        <p class="text-[11px] text-slate-500 max-w-xs">
                            {"Latar belakang merah polos, jas dokter snelli putih bersih, dasi rapi, wajah tampak depan 80%."}
                        </p>
                    </div>

                    <button 
                        type="button" 
                        onclick={on_open_photo_modal}
                        class="w-full py-2 rounded-xl bg-slate-50 hover:bg-slate-100 border border-slate-200 text-xs font-bold text-slate-700 transition-colors flex items-center justify-center gap-1.5"
                    >
                        <span class="material-symbols-outlined text-[16px] text-[#006a61]">{"photo_camera"}</span>
                        <span>{"Kelola / Pratinjau Foto"}</span>
                    </button>
                </div>

                <div class="lg:col-span-2 p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col justify-between gap-6">
                    <div>
                        <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                            <div class="flex items-center gap-2">
                                <span class="material-symbols-outlined text-[#006a61] text-[24px]">{"verified"}</span>
                                <h3 class="text-base font-bold text-[#001428]">
                                    {"Integritas & Status Kurasi Dokumen"}
                                </h3>
                            </div>
                            <span class="px-3 py-1 rounded-full bg-emerald-50 text-emerald-700 border border-emerald-200 text-xs font-bold">
                                {"100% Terpenuhi (9/9 Berkas)"}
                            </span>
                        </div>

                        <div class="mt-4 flex flex-col gap-1.5">
                            <div class="flex justify-between text-xs font-semibold">
                                <span class="text-slate-600">{"Progres Kelengkapan Persyaratan Wajib"}</span>
                                <span class="text-[#006a61]">{"9 dari 9 Berkas Valid"}</span>
                            </div>
                            <div class="w-full h-2.5 rounded-full bg-slate-100 overflow-hidden">
                                <div class="h-full bg-[#006a61] rounded-full w-full"></div>
                            </div>
                        </div>

                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mt-6">
                            <div class="p-3 rounded-xl bg-slate-50 border border-slate-200 flex flex-col">
                                <span class="text-[10px] uppercase font-bold text-slate-400">{"Total Berkas"}</span>
                                <span class="text-lg font-bold text-slate-800">{"9 Berkas"}</span>
                            </div>
                            <div class="p-3 rounded-xl bg-emerald-50 border border-emerald-100 flex flex-col">
                                <span class="text-[10px] uppercase font-bold text-emerald-600">{"Status Valid"}</span>
                                <span class="text-lg font-bold text-emerald-700">{"9 Dokumen"}</span>
                            </div>
                            <div class="p-3 rounded-xl bg-slate-50 border border-slate-200 flex flex-col">
                                <span class="text-[10px] uppercase font-bold text-slate-400">{"Revisi Panitia"}</span>
                                <span class="text-lg font-bold text-slate-800">{"0 Dokumen"}</span>
                            </div>
                            <div class="p-3 rounded-xl bg-teal-50 border border-teal-100 flex flex-col">
                                <span class="text-[10px] uppercase font-bold text-[#006a61]">{"TTE BSrE Valid"}</span>
                                <span class="text-lg font-bold text-[#006a61]">{"Terverifikasi"}</span>
                            </div>
                        </div>
                    </div>

                    <div class="p-4 rounded-xl bg-slate-50 border border-slate-200 flex items-center justify-between text-xs text-slate-600">
                        <div class="flex items-center gap-2">
                            <span class="material-symbols-outlined text-[18px] text-[#006a61]">{"security"}</span>
                            <span>{"Seluruh dokumen dipindai dari ijazah asli dan memiliki stempel hash digital."}</span>
                        </div>
                    </div>
                </div>
            </div>

            <section class="p-6 rounded-2xl bg-white border border-slate-200 shadow-sm flex flex-col gap-4">
                <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                    <h2 class="text-base font-bold text-[#001428]">{"Daftar 9 Berkas Persyaratan Administrasi Wajib"}</h2>
                    <span class="text-xs text-slate-400">{"Klik icon mata untuk melihat dokumen & tanda tangan digital"}</span>
                </div>

                <div class="overflow-x-auto">
                    <table class="w-full text-left text-xs">
                        <thead>
                            <tr class="border-b border-slate-200 text-slate-400 font-bold uppercase tracking-wider text-[10px]">
                                <th class="pb-3 px-3">{"Nama Dokumen Persyaratan"}</th>
                                <th class="pb-3 px-3">{"Format & Ukuran"}</th>
                                <th class="pb-3 px-3">{"Waktu Unggah"}</th>
                                <th class="pb-3 px-3">{"Status Verifikasi"}</th>
                                <th class="pb-3 px-3">{"Catatan Panitia"}</th>
                                <th class="pb-3 px-3 text-right">{"Tindakan"}</th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-slate-100">
                            {
                                for documents.iter().map(|doc| {
                                    let doc_clone = doc.clone();
                                    let doc_reupload = doc.clone();
                                    let on_preview = {
                                        let on_preview_doc = on_preview_doc.clone();
                                        Callback::from(move |_| {
                                            on_preview_doc.emit(doc_clone.clone());
                                        })
                                    };
                                    let on_reupload = {
                                        let on_reupload_click = on_reupload_click.clone();
                                        Callback::from(move |_| {
                                            on_reupload_click.emit(doc_reupload.title.clone());
                                        })
                                    };

                                    html! {
                                        <tr class="hover:bg-slate-50 transition-colors">
                                            <td class="py-3.5 px-3">
                                                <div class="flex items-center gap-3">
                                                    <div class="w-9 h-9 rounded-lg bg-slate-100 text-[#006a61] flex items-center justify-center shrink-0">
                                                        <span class="material-symbols-outlined text-[20px]">{&doc.icon}</span>
                                                    </div>
                                                    <div class="flex flex-col">
                                                        <span class="font-bold text-slate-800 text-xs sm:text-sm">{&doc.title}</span>
                                                        <span class="text-[11px] text-slate-400">{&doc.subtitle}</span>
                                                    </div>
                                                </div>
                                            </td>
                                            <td class="py-3.5 px-3 font-mono">
                                                <span class="px-1.5 py-0.5 rounded bg-slate-100 font-bold text-slate-700 mr-1.5">{&doc.format}</span>
                                                <span class="text-slate-500">{&doc.size}</span>
                                            </td>
                                            <td class="py-3.5 px-3 text-slate-500">
                                                {&doc.upload_date}
                                            </td>
                                            <td class="py-3.5 px-3">
                                                <span class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-emerald-50 text-emerald-700 border border-emerald-200 font-bold text-[11px]">
                                                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-600"></span>
                                                    {&doc.status}
                                                </span>
                                            </td>
                                            <td class="py-3.5 px-3 text-slate-600">
                                                {&doc.note}
                                            </td>
                                            <td class="py-3.5 px-3 text-right">
                                                <div class="inline-flex items-center gap-1">
                                                    <button 
                                                        type="button" 
                                                        onclick={on_preview}
                                                        class="p-1.5 rounded-lg text-[#006a61] hover:bg-teal-50 transition-colors"
                                                        title="Pratinjau Dokumen"
                                                    >
                                                        <span class="material-symbols-outlined text-[18px]">{"visibility"}</span>
                                                    </button>
                                                    <button 
                                                        type="button" 
                                                        onclick={on_reupload}
                                                        class="p-1.5 rounded-lg text-slate-400 hover:text-slate-700 hover:bg-slate-100 transition-colors"
                                                        title="Unggah Ulang"
                                                    >
                                                        <span class="material-symbols-outlined text-[18px]">{"sync"}</span>
                                                    </button>
                                                </div>
                                            </td>
                                        </tr>
                                    }
                                })
                            }
                        </tbody>
                    </table>
                </div>
            </section>

            {
                if let Some(doc) = (*selected_doc).as_ref() {
                    let on_close = on_close_doc_modal.clone();
                    let doc_title = doc.title.clone();
                    let on_download = Callback::from(move |_| {
                        gloo_dialogs::alert(&format!("Mengunduh salinan resmi berkas '{}'...", doc_title));
                    });

                    html! {
                        <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                            <div class="bg-white rounded-2xl max-w-lg w-full p-6 shadow-2xl border border-slate-200 flex flex-col gap-4 animate-in fade-in zoom-in-95 duration-150">
                                <div class="flex items-center justify-between pb-3 border-b border-slate-100">
                                    <div class="flex items-center gap-2">
                                        <span class="material-symbols-outlined text-[#006a61] text-[24px]">{"description"}</span>
                                        <h3 class="text-base font-bold text-[#001428]">{"Pratinjau Dokumen Terverifikasi"}</h3>
                                    </div>
                                    <button type="button" onclick={on_close.clone()} class="text-slate-400 hover:text-slate-700">
                                        <span class="material-symbols-outlined text-[20px]">{"close"}</span>
                                    </button>
                                </div>

                                <div class="flex flex-col gap-2">
                                    <h4 class="text-sm font-bold text-slate-800">{&doc.title}</h4>
                                    <span class="text-xs text-slate-500 font-mono">
                                        {format!("Ukuran: {} • Format: {} • Status: Terverifikasi", &doc.size, &doc.format)}
                                    </span>
                                    <p class="text-xs text-slate-600 mt-1 leading-relaxed">
                                        {&doc.description}
                                    </p>
                                </div>

                                <div class="p-3 rounded-xl bg-slate-50 border border-slate-200 flex flex-col gap-1 text-xs">
                                    <div class="flex items-center gap-1.5 text-emerald-700 font-bold">
                                        <span class="material-symbols-outlined text-[16px]">{"verified"}</span>
                                        <span>{"Tanda Tangan Digital Kemenkes / KKI Valid (BSrE)"}</span>
                                    </div>
                                    <span class="text-[11px] text-slate-500 font-mono break-all mt-1">
                                        {format!("SHA-256: {}", &doc.sha256)}
                                    </span>
                                </div>

                                <div class="flex items-center justify-between pt-3 border-t border-slate-100">
                                    <span class="text-[11px] text-slate-400">{"Audit Log: Kemenkes SDMK Valid"}</span>
                                    <div class="flex items-center gap-2">
                                        <button 
                                            type="button" 
                                            onclick={on_close} 
                                            class="px-4 py-2 rounded-xl text-xs font-bold text-slate-600 hover:bg-slate-100 transition-colors"
                                        >
                                            {"Tutup"}
                                        </button>
                                        <button 
                                            type="button" 
                                            onclick={on_download}
                                            class="px-4 py-2 rounded-xl text-xs font-bold bg-[#0f2942] text-white hover:bg-[#001428] flex items-center gap-1.5 transition-colors shadow-sm"
                                        >
                                            <span class="material-symbols-outlined text-[16px]">{"download"}</span>
                                            <span>{"Unduh Berkas"}</span>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            {
                if *is_photo_modal_open {
                    let on_close_photo = on_close_photo_modal.clone();
                    let on_upload_photo = Callback::from(|_| {
                        gloo_dialogs::alert("Fitur unggah pasfoto baru terbuka. Memvalidasi kepatuhan rasio 4x6 dan latar belakang merah...");
                    });

                    html! {
                        <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4">
                            <div class="bg-white rounded-2xl max-w-sm w-full p-6 shadow-2xl border border-slate-200 flex flex-col items-center gap-4 text-center">
                                <div class="flex items-center justify-between w-full pb-2 border-b border-slate-100">
                                    <h3 class="text-sm font-bold text-[#001428]">{"Pratinjau Pasfoto Resmi KKI"}</h3>
                                    <button type="button" onclick={on_close_photo.clone()} class="text-slate-400 hover:text-slate-700">
                                        <span class="material-symbols-outlined text-[20px]">{"close"}</span>
                                    </button>
                                </div>

                                <div class="w-44 h-56 rounded-xl overflow-hidden shadow-lg ring-4 ring-slate-200 bg-[#b91c1c] flex items-center justify-center">
                                    <img 
                                        src="https://lh3.googleusercontent.com/aida-public/AB6AXuB3iXfipsSIXgK7xUIK4nU8d_47ZekCMHCWjMonjVpa82n4E44GSPl3sseiCD_PRAv24auNSSHOMYKFPbrYGMmhF4ewg1ycxW4bpUlShdDS1DEpNix1Z1AoZQVDR7DHlfylnvjOM2ebMYkWAGoaPfKBpeVgHkk9xTFxbnFI0gMsklk17lXi_V-AZyHfFMiKvgSsI7UCep74_sXuVApxfvNE_A10yA0go4XPzwzV2Rt8CUv1JidvJLRM" 
                                        alt="Pasfoto Dokter" 
                                        class="w-full h-full object-cover object-top"
                                    />
                                </div>

                                <div class="text-xs text-slate-500">
                                    <span class="font-bold text-slate-800 block">{"dr. Agung Setiawan, S.Ked"}</span>
                                    {"Status: Terverifikasi Standar KKI & PD-Dikti"}
                                </div>

                                <div class="flex items-center gap-2 w-full pt-2">
                                    <button 
                                        type="button" 
                                        onclick={on_close_photo} 
                                        class="flex-1 py-2 rounded-xl text-xs font-bold text-slate-600 hover:bg-slate-100 border border-slate-200"
                                    >
                                        {"Kembali"}
                                    </button>
                                    <button 
                                        type="button" 
                                        onclick={on_upload_photo} 
                                        class="flex-1 py-2 rounded-xl text-xs font-bold bg-[#006a61] text-white hover:bg-[#005049] flex items-center justify-center gap-1 shadow-sm"
                                    >
                                        <span class="material-symbols-outlined text-[16px]">{"upload"}</span>
                                        <span>{"Ganti Foto"}</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
