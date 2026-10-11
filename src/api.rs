use crate::models::*;

pub struct ApiService;

impl ApiService {
    pub const SUPABASE_URL: &'static str = "https://xbozhlpnznpllxdkcaor.supabase.co";
    pub const SUPABASE_ANON_KEY: &'static str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Inhib3pobHBuem5wbGx4ZGtjYW9yIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NjQyMDI2NTcsImV4cCI6MjA3OTc3ODY1N30.oPB3xwhWk_NYTjUNw3bHjzro1gEr8nlt-7kuvQL8GEg";

    pub fn get_candidate_profile() -> CandidateProfile {
        CandidateProfile::default()
    }

    pub fn get_selection_stages() -> Vec<StageItem> {
        vec![
            StageItem {
                stage_number: 1,
                title: "Pendaftaran Akun".to_string(),
                subtitle: "Tahap 1 • Selesai".to_string(),
                date: "12 Juli 2025".to_string(),
                description: "Formulir lengkap & biodata tervalidasi NPA IDI.".to_string(),
                status: StageStatus::Completed,
            },
            StageItem {
                stage_number: 2,
                title: "Verifikasi Berkas".to_string(),
                subtitle: "Tahap 2 • Selesai".to_string(),
                date: "20 Juli 2025".to_string(),
                description: "8/8 Dokumen lolos kurasi Kemenkes RI.".to_string(),
                status: StageStatus::Completed,
            },
            StageItem {
                stage_number: 3,
                title: "Seleksi Administrasi".to_string(),
                subtitle: "Tahap 3 • Selesai".to_string(),
                date: "25 Juli 2025".to_string(),
                description: "Skor Memenuhi Syarat & Portofolio Disetujui.".to_string(),
                status: StageStatus::Completed,
            },
            StageItem {
                stage_number: 4,
                title: "Ujian Tulis & CBT".to_string(),
                subtitle: "Tahap 4 • Selesai".to_string(),
                date: "02 Agustus 2025".to_string(),
                description: "Nilai CBT: 84.20 (Standar Kelulusan >= 75.00).".to_string(),
                status: StageStatus::Completed,
            },
            StageItem {
                stage_number: 5,
                title: "Wawancara & MMPI".to_string(),
                subtitle: "Tahap 5 • Jadwal Aktif".to_string(),
                date: "18 Agustus 2025".to_string(),
                description: "Wawancara dewan panelis Sp.An-TI FK UNTAN.".to_string(),
                status: StageStatus::Current,
            },
            StageItem {
                stage_number: 6,
                title: "Yudisium Akhir".to_string(),
                subtitle: "Tahap 6 • Menunggu".to_string(),
                date: "28 Agustus 2025".to_string(),
                description: "Penetapan Surat Keputusan Rektor UNTAN.".to_string(),
                status: StageStatus::Upcoming,
            },
        ]
    }

    pub fn get_interview_schedule() -> InterviewSchedule {
        InterviewSchedule {
            date_formatted: "Senin, 18 Agustus 2025".to_string(),
            time_formatted: "08:30 - 11:45 WIB".to_string(),
            location: "Gedung Dekanat FK Universitas Tanjungpura, Lantai 3".to_string(),
            room: "Ruang Sidang Utama KATI (Konsorsium Anestesi)".to_string(),
            panel_title: "Dewan Penguji Panelis Spesialis Anestesiologi & Terapi Intensif".to_string(),
            panel_members: vec![
                "Prof. Dr. dr. M. Alamsyah, Sp.An-KIC (Ketua Program Studi)".to_string(),
                "Dr. dr. Hendra Suryanto, Sp.An-KAR (Kepala KSM Anestesi RSUD Dr. Soedarso)".to_string(),
                "dr. Ratna Dewi, Sp.An-TI, M.Kes (Sekretaris Kolegium Anestesiologi)".to_string(),
            ],
        }
    }

    pub fn get_evaluations() -> Vec<EvaluationScore> {
        vec![
            EvaluationScore {
                component_name: "Tes Potensi Akademik (TPA Bappenas)".to_string(),
                category: "Akademik".to_string(),
                score_numeric: Some(592.50),
                score_qualitative: Some("Memenuhi Ambang Batas (Min. 500)".to_string()),
                status: "Lulus".to_string(),
            },
            EvaluationScore {
                component_name: "CBT Kemampuan Dasar Kedokteran".to_string(),
                category: "Akademik".to_string(),
                score_numeric: Some(84.20),
                score_qualitative: Some("Peringkat 4 Regional".to_string()),
                status: "Lulus".to_string(),
            },
            EvaluationScore {
                component_name: "Psikotes & MMPI-2 Klinis".to_string(),
                category: "Kesehatan Mental".to_string(),
                score_numeric: None,
                score_qualitative: Some("Profil Kepribadian Stabil • Valid".to_string()),
                status: "Rekomendasi Penuh".to_string(),
            },
            EvaluationScore {
                component_name: "Pemeriksaan Fisik & Bebas Narkoba".to_string(),
                category: "Kesehatan Fisik".to_string(),
                score_numeric: None,
                score_qualitative: Some("Fit to Practice & Non-Reactive".to_string()),
                status: "Lulus Medis".to_string(),
            },
        ]
    }

    pub fn get_documents() -> Vec<DocumentItem> {
        vec![
            DocumentItem {
                id: "doc-1".to_string(),
                title: "Ijazah & Transkrip Sarjana Kedokteran (S.Ked)".to_string(),
                subtitle: "Universitas Airlangga (FK UNAIR) • IPK: 3.65".to_string(),
                format: "PDF".to_string(),
                size: "2.4 MB".to_string(),
                upload_date: "12 Feb 2025, 09:30 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "Legalisir Dekanat & Terverifikasi PD-Dikti".to_string(),
                icon: "school".to_string(),
                description: "Salinan Ijazah Sarjana Kedokteran dan Transkrip Akademik Kumulatif terlegalisir basah dan e-Sign BSrE.".to_string(),
                sha256: "8a4c9b107e32fa998c1145bc90e876fa123bca0128749d43e201ba9241517cc0".to_string(),
            },
            DocumentItem {
                id: "doc-2".to_string(),
                title: "Ijazah & Transkrip Profesi Dokter (dr.)".to_string(),
                subtitle: "Universitas Airlangga (FK UNAIR) • IPK: 3.78".to_string(),
                format: "PDF".to_string(),
                size: "1.8 MB".to_string(),
                upload_date: "12 Feb 2025, 09:34 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "Legalisir Dekanat & Terverifikasi PD-Dikti".to_string(),
                icon: "medical_services".to_string(),
                description: "Ijazah Profesi Dokter dan Transkrip Nilai Kepaniteraan Klinik Stase Lengkap.".to_string(),
                sha256: "1f8e432190cba7654321fedcba0987654321fedcba0987654321fedcba098765".to_string(),
            },
            DocumentItem {
                id: "doc-3".to_string(),
                title: "Surat Tanda Registrasi (STR) Dokter Aktif".to_string(),
                subtitle: "Konsil Kedokteran Indonesia (KKI) • No: 31.1.1.100.1.20.219842".to_string(),
                format: "PDF".to_string(),
                size: "680 KB".to_string(),
                upload_date: "12 Feb 2025, 09:41 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "QR-Code Tervalidasi SatuSehat SDMK s/d 2028".to_string(),
                icon: "verified_user".to_string(),
                description: "Surat Tanda Registrasi Dokter dari Konsil Kedokteran Indonesia berlaku aktif hingga 14 November 2028.".to_string(),
                sha256: "5d41402abc4b2a76b9719d911017c592be5a043e201ba9241517cc0892019ab3".to_string(),
            },
            DocumentItem {
                id: "doc-4".to_string(),
                title: "Sertifikat Kelulusan UKMPPD / STSI".to_string(),
                subtitle: "Komite Bersama Uji Kompetensi Mahasiswa Kedokteran".to_string(),
                format: "PDF".to_string(),
                size: "950 KB".to_string(),
                upload_date: "12 Feb 2025, 09:48 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "CBT: 86.50 • OSCE: 88.00 (First Taker)".to_string(),
                icon: "workspace_premium".to_string(),
                description: "Sertifikat Tanda Lulus Uji Kompetensi Mahasiswa Program Profesi Dokter nasional.".to_string(),
                sha256: "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca7".to_string(),
            },
            DocumentItem {
                id: "doc-5".to_string(),
                title: "Sertifikat ACLS / ATLS Aktif".to_string(),
                subtitle: "PERKI & IKABI Pusat".to_string(),
                format: "PDF".to_string(),
                size: "1.1 MB".to_string(),
                upload_date: "12 Feb 2025, 09:55 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "Masa berlaku s/d November 2027".to_string(),
                icon: "cardiology".to_string(),
                description: "Sertifikasi Advanced Cardiac Life Support dan Advanced Trauma Life Support terakreditasi Kemenkes RI.".to_string(),
                sha256: "7215ee9c7d9dc229d2921a40e899ec5f76b3ddec11d7028cfb7940029a5322b2".to_string(),
            },
            DocumentItem {
                id: "doc-6".to_string(),
                title: "Surat Keterangan Sehat Fisik RS Tipe A/B".to_string(),
                subtitle: "RSUP Dr. Cipto Mangunkusumo Jakarta".to_string(),
                format: "PDF".to_string(),
                size: "750 KB".to_string(),
                upload_date: "13 Feb 2025, 11:02 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "Ditandatangani Dokter Penguji Spesialis Okupasi".to_string(),
                icon: "health_and_safety".to_string(),
                description: "Hasil pemeriksaan fisik lengkap MCU Calon Peserta PPDS Rumah Sakit Tipe A.".to_string(),
                sha256: "3ac340832f29c11538fbe2d6f75e8d3888fa4a993fd9622e32fa86ea1a20790e".to_string(),
            },
            DocumentItem {
                id: "doc-7".to_string(),
                title: "Surat Bebas Narkoba & Bebas Buta Warna".to_string(),
                subtitle: "Laboratorium Patologi Klinik RSUP Fatmawati".to_string(),
                format: "PDF".to_string(),
                size: "620 KB".to_string(),
                upload_date: "13 Feb 2025, 11:24 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "Uji Ishihara 38 Plates: Normal • Narkoba Negatif".to_string(),
                icon: "science".to_string(),
                description: "Panel skrining toksikologi 6 parameter dan uji persepsi warna Ishihara.".to_string(),
                sha256: "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb".to_string(),
            },
            DocumentItem {
                id: "doc-8".to_string(),
                title: "Surat Rekomendasi IDI Cabang & Etik Kedokteran".to_string(),
                subtitle: "Ikatan Dokter Indonesia Cabang Pontianak".to_string(),
                format: "PDF".to_string(),
                size: "480 KB".to_string(),
                upload_date: "13 Feb 2025, 11:30 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "Surat Bebas Pelanggaran Etik No: 204/IDI-PTK/II/2025".to_string(),
                icon: "assignment_turned_in".to_string(),
                description: "Surat Keterangan Kepatuhan Etika Profesi Kedokteran diterbitkan oleh Ketua IDI Cabang Pontianak.".to_string(),
                sha256: "8e9f2982ef2bda88147b7b1b3ac92019aeb3f7614d9b010839201a0984920194".to_string(),
            },
            DocumentItem {
                id: "doc-9".to_string(),
                title: "Sertifikat Kemampuan Bahasa Inggris (TOEFL ITP)".to_string(),
                subtitle: "Pusat Bahasa Universitas Tanjungpura (UNTAN)".to_string(),
                format: "PDF".to_string(),
                size: "510 KB".to_string(),
                upload_date: "13 Feb 2025, 11:35 WIB".to_string(),
                status: "Valid".to_string(),
                status_color: "secondary".to_string(),
                note: "Skor: 547 (Standar Minimal FK UNTAN: 475)".to_string(),
                icon: "language".to_string(),
                description: "Sertifikat resmi Institutional Testing Program (ITP) TOEFL terakreditasi IIEF/ETS.".to_string(),
                sha256: "f4b0028a3f019bd76498a4d70102143bc09121a978502390a42e128490a09321".to_string(),
            },
        ]
    }

    pub fn get_announcements() -> Vec<AnnouncementItem> {
        vec![
            AnnouncementItem {
                id: "ann-1".to_string(),
                title: "Penerbitan Jadwal Wawancara Resmi Tahap 3".to_string(),
                content: "Jadwal panel spesifik per kelompok telah dikirim melalui email peserta dan tampil di dashboard.".to_string(),
                date: "11 Agustus 2025 • 14:20 WIB".to_string(),
                is_highlight: true,
            },
            AnnouncementItem {
                id: "ann-2".to_string(),
                title: "Hasil Rekap MMPI & Tes Bebas Narkoba Telah Terverifikasi".to_string(),
                content: "Seluruh berkas kesehatan telah diunggah ke sistem database bersama RS Kariadi-UNDIP & FK UNTAN.".to_string(),
                date: "10 Agustus 2025 • 17:00 WIB".to_string(),
                is_highlight: false,
            },
        ]
    }
}
