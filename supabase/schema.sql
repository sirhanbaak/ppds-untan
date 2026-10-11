-- ==============================================================================
-- SUPABASE DATABASE SCHEMA: PORTAL SELEKSI PPDS FK UNTAN
-- ==============================================================================
-- Reverse-engineered from:
--   1. index.html (Beranda & Status Seleksi, Stepper Pipeline, Rekapitulasi Nilai)
--   2. formulir_pendaftaran.html (Biodata, Program Studi, Pengalaman Klinis, Sponsor)
--   3. unggah_berkas.html (Pasfoto Dokter & 9 Berkas Persyaratan Administrasi Medis)
--   4. hasil_pengumuman.html (Pengumuman Yudisium Tahap 2, SK Rektor/Dekan, Wawancara)
--
-- Target Database: Supabase PostgreSQL (Postgres 15+)
-- Features:
--   - Strict relational integrity & constraints
--   - Role-Based Access Control (Candidate, Verifier, Assessor, Interviewer, Admin)
--   - Comprehensive Row Level Security (RLS) policies
--   - Immutability enforcement on locked applications
--   - Storage Buckets configuration & Storage RLS
--   - Realtime-ready triggers & Views for dashboard widgets
-- ==============================================================================

-- ------------------------------------------------------------------------------
-- 1. EXTENSIONS
-- ------------------------------------------------------------------------------
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ------------------------------------------------------------------------------
-- 2. ENUMS & CUSTOM DOMAINS
-- ------------------------------------------------------------------------------
CREATE TYPE public.user_role AS ENUM (
    'candidate',      -- Calon Residen Dokter
    'verifier',       -- Panitia Seleksi Berkas Administrasi
    'assessor',       -- Penguji Akademik / CBT / MCU / Psikometri
    'interviewer',    -- Penguji Wawancara Panelis Kolegium & KPS
    'admin',          -- Administrator Seleksi PPDS / Dekanat
    'super_admin'     -- Super Admin IT Sistem
);

CREATE TYPE public.gender_type AS ENUM (
    'laki-laki',
    'perempuan'
);

CREATE TYPE public.blood_type AS ENUM (
    'A+', 'A-', 'B+', 'B-', 'AB+', 'AB-', 'O+', 'O-'
);

CREATE TYPE public.period_status AS ENUM (
    'draft',          -- Draf periode belum dibuka
    'open',           -- Pendaftaran dibuka
    'stage_1_review', -- Verifikasi Berkas Administrasi
    'stage_2_exam',   -- Pelaksanaan CBT & MCU
    'stage_3_interview', -- Pelaksanaan Wawancara
    'completed',      -- Yudisium Akhir Selesai
    'archived'        -- Arsip periode lampau
);

CREATE TYPE public.admission_path AS ENUM (
    'reguler_nasional',   -- Jalur Reguler Nasional
    'tubel_kemenkes',     -- Tugas Belajar Kemenkes RI
    'mitra_rs_pemda',     -- Afiliasi Daerah / Bantuan Pemda & Mitra RS
    'afirmasi_daerah'     -- Afiliasi Daerah Prioritas Kalbar
);

CREATE TYPE public.funding_type AS ENUM (
    'lpdp',          -- Beasiswa LPDP Targeted Dokter Spesialis
    'kemenkes',      -- Beasiswa Kemenkes RI
    'pemda',         -- Beasiswa Pemda Provinsi/Kabupaten
    'mitra_rs',      -- Ikatan Dinas Rumah Sakit
    'mandiri'        -- Biaya Mandiri
);

CREATE TYPE public.application_status AS ENUM (
    'draft',                 -- Draf pendaftaran belum dikunci
    'submitted',             -- Terkunci & Menunggu Verifikasi
    'passed_stage_1',        -- Lolos Seleksi Administrasi (Tahap 1)
    'failed_stage_1',        -- Gugur di Seleksi Administrasi
    'passed_stage_2',        -- Lolos Akademik & MCU (Tahap 2)
    'failed_stage_2',        -- Gugur di Tahap 2
    'interview_scheduled',   -- Menunggu Pelaksanaan Wawancara (Tahap 3)
    'passed_final',          -- LULUS PPDS (Penetapan Yudisium Final)
    'rejected_final'         -- Tidak Lulus Yudisium Akhir
);

CREATE TYPE public.verification_status AS ENUM (
    'pending',               -- Belum diverifikasi
    'valid',                 -- Terverifikasi & Diterima
    'needs_revision',        -- Perlu revisi/unggah ulang
    'rejected'               -- Berkas ditolak tidak sah
);

-- ------------------------------------------------------------------------------
-- 3. CORE MASTER DATA TABLES
-- ------------------------------------------------------------------------------

-- Master Table: Selection Periods (e.g. Periode Ganjil 2025/2026 Gelombang I)
CREATE TABLE public.admission_periods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    academic_year VARCHAR(20) NOT NULL,            -- e.g. '2025/2026'
    semester VARCHAR(10) NOT NULL,                 -- e.g. 'Ganjil'
    batch_name VARCHAR(100) NOT NULL,              -- e.g. 'Gelombang I (Jalur Utama)'
    status public.period_status NOT NULL DEFAULT 'open',
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    cbt_exam_date DATE,                            -- e.g. '2025-07-18'
    cbt_exam_location TEXT DEFAULT 'Pusat Asesmen CBT FK UNTAN Pontianak',
    stage_1_announcement_date TIMESTAMPTZ,         -- e.g. '2025-07-25'
    stage_2_announcement_date TIMESTAMPTZ,         -- e.g. '2025-08-12'
    interview_start_date TIMESTAMPTZ,              -- e.g. '2025-08-18'
    final_announcement_date TIMESTAMPTZ,           -- e.g. '2025-08-25'
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Master Table: Study Programs (e.g. Sp.An-TI FK UNTAN)
CREATE TABLE public.study_programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,              -- e.g. 'SP-031-ANESTI'
    name VARCHAR(255) NOT NULL,                    -- e.g. 'Spesialis Anestesiologi dan Terapi Intensif'
    degree_title VARCHAR(50) NOT NULL,             -- e.g. 'Sp.An-TI'
    level VARCHAR(50) NOT NULL DEFAULT 'PPDS-1',
    standard_semesters INT NOT NULL DEFAULT 8,
    faculty VARCHAR(255) NOT NULL DEFAULT 'Fakultas Kedokteran Universitas Tanjungpura',
    collegium_name VARCHAR(255) NOT NULL,          -- e.g. 'Kolegium Anestesiologi dan Terapi Intensif Indonesia (KATI)'
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Master Table: Teaching Hospitals (RS Jejaring Pendidikan Dokter Spesialis)
CREATE TABLE public.teaching_hospitals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,                    -- e.g. 'RSUD Dokter Soedarso Pontianak'
    hospital_type VARCHAR(100) NOT NULL,           -- e.g. 'Rumah Sakit Pendidikan Utama (Tipe A)'
    city VARCHAR(100) NOT NULL DEFAULT 'Pontianak',
    province VARCHAR(100) NOT NULL DEFAULT 'Kalimantan Barat',
    description TEXT,                              -- e.g. 'Pusat Trauma, ICU, & Bedah Sentral IBS'
    is_main_hospital BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Junction Table: Program to Hospitals (Afiliasi RS jejaring untuk prodi)
CREATE TABLE public.program_hospital_affiliations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    program_id UUID NOT NULL REFERENCES public.study_programs(id) ON DELETE CASCADE,
    hospital_id UUID NOT NULL REFERENCES public.teaching_hospitals(id) ON DELETE CASCADE,
    is_main_center BOOLEAN NOT NULL DEFAULT false,
    rotation_notes TEXT,                           -- e.g. 'Stase Anestesi Bedah Umum & Subspesialistik, ICU/ICCU'
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_program_hospital UNIQUE(program_id, hospital_id)
);

-- Period-Program Offering & Quota (e.g. Daya Tampung: 2 Kursi Residen di Periode Ganjil 2025/2026)
CREATE TABLE public.period_programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID NOT NULL REFERENCES public.admission_periods(id) ON DELETE CASCADE,
    program_id UUID NOT NULL REFERENCES public.study_programs(id) ON DELETE CASCADE,
    quota_seats INT NOT NULL DEFAULT 2,            -- Daya tampung resmi (2 kursi)
    is_single_choice BOOLEAN NOT NULL DEFAULT true,-- Prototype indicates Sp.An-TI is single choice in UNTAN
    is_open BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_period_program UNIQUE(period_id, program_id)
);

-- Master Table: Required Document Definitions (The 9 Mandatory Documents in unggah_berkas.html)
CREATE TABLE public.document_requirements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,              -- e.g. 'DOC_IJAZAH_TRANSKRIP'
    name VARCHAR(255) NOT NULL,                    -- e.g. 'Ijazah Dokter & Transkrip Akademik Terlegalisir'
    description TEXT,
    allowed_format VARCHAR(20) NOT NULL DEFAULT 'PDF',
    max_size_kb INT NOT NULL DEFAULT 2048,         -- 2 MB
    is_mandatory BOOLEAN NOT NULL DEFAULT true,
    order_index INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Master Table: Selection Stages Definitions (The 6 Pipeline Stepper Stages)
CREATE TABLE public.selection_stages (
    stage_number INT PRIMARY KEY,                  -- 1 to 6
    title VARCHAR(100) NOT NULL,                   -- e.g. 'Wawancara Panelis'
    subtitle VARCHAR(255),
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ------------------------------------------------------------------------------
-- 4. USER PROFILES & AUTH EXTENSION
-- ------------------------------------------------------------------------------
-- Links 1-to-1 with Supabase auth.users
CREATE TABLE public.profiles (
    id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    role public.user_role NOT NULL DEFAULT 'candidate',
    full_name VARCHAR(255) NOT NULL,               -- e.g. 'dr. Agung Setiawan, S.Ked'
    nik VARCHAR(16) UNIQUE,                        -- e.g. '3374021405940003'
    place_of_birth VARCHAR(100),                   -- e.g. 'Semarang'
    date_of_birth DATE,                            -- e.g. '1994-05-14'
    gender public.gender_type,
    blood_group public.blood_type,
    phone_number VARCHAR(30),                      -- e.g. '+62 812-3498-7712'
    address TEXT,
    
    -- KKI & IDI Medical Credentials
    str_number VARCHAR(100),                       -- e.g. '33.1.1.100.2.20.187293'
    str_valid_until DATE,                          -- e.g. '2028-05-14'
    str_is_lifetime BOOLEAN DEFAULT false,
    str_is_verified BOOLEAN DEFAULT false,
    idi_number VARCHAR(100),                       -- e.g. 'NPA-142089'
    idi_branch VARCHAR(100),                       -- e.g. 'Kota Semarang, Jawa Tengah'
    idi_is_verified BOOLEAN DEFAULT false,

    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ------------------------------------------------------------------------------
-- 5. CANDIDATE APPLICATION & REGISTRATION DATA
-- ------------------------------------------------------------------------------

-- Core Table: Candidate Applications
CREATE TABLE public.applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    registration_number VARCHAR(50) UNIQUE NOT NULL, -- e.g. 'REG-PPDS-2025-0892' or '2025/REG-ANESTI/0892'
    user_id UUID NOT NULL REFERENCES public.profiles(id) ON DELETE CASCADE,
    period_program_id UUID NOT NULL REFERENCES public.period_programs(id) ON DELETE RESTRICT,
    
    -- Admission & Funding Details (Formulir Section 1 & 4)
    admission_path public.admission_path NOT NULL DEFAULT 'reguler_nasional',
    funding_type public.funding_type NOT NULL DEFAULT 'lpdp',
    funding_scheme_name VARCHAR(255) NOT NULL DEFAULT 'Beasiswa LPDP Targeted Dokter Spesialis',
    funding_category VARCHAR(255) DEFAULT 'Dokter Spesialis Prioritas Kemenkes RI',
    funding_document_status VARCHAR(100) DEFAULT 'Letter of Sponsorship (LoS) Siap',
    backup_funding_source VARCHAR(255) DEFAULT 'Mandiri / Rekening Terverifikasi Bank Himbara',

    -- Status & Progression
    current_stage INT NOT NULL DEFAULT 1 REFERENCES public.selection_stages(stage_number),
    overall_status public.application_status NOT NULL DEFAULT 'draft',
    is_locked BOOLEAN NOT NULL DEFAULT false,        -- Terkunci & Terverifikasi Panitia
    locked_at TIMESTAMPTZ,
    
    -- Rank & Cohort Data
    rank INT,                                        -- e.g. Peringkat 4
    total_cohort_candidates INT DEFAULT 86,          -- Dari 86 kandidat

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_user_period_application UNIQUE(user_id, period_program_id)
);

-- Table: Medical Academic History (Formulir Section 2 Almamater)
CREATE TABLE public.applicant_academic_histories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID UNIQUE NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    almamater_name VARCHAR(255) NOT NULL,            -- e.g. 'Fakultas Kedokteran Universitas Airlangga (UNAIR) Surabaya'
    graduation_year INT NOT NULL,                    -- e.g. 2019
    gpa_bachelor NUMERIC(3,2) NOT NULL,              -- e.g. 3.65
    gpa_profession NUMERIC(3,2) NOT NULL,            -- e.g. 3.78
    predicate VARCHAR(100) DEFAULT 'Predikat Sangat Memuaskan',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table: Clinical Experiences & Rotations (Formulir Section 3: PIDI & Dokter Jaga IGD)
CREATE TABLE public.applicant_clinical_experiences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    experience_type VARCHAR(50) NOT NULL,            -- 'internship' or 'clinical_job'
    title VARCHAR(255) NOT NULL,                     -- e.g. 'Program Internship Dokter Indonesia (PIDI)'
    duration_label VARCHAR(100) NOT NULL,            -- e.g. 'Mei 2019 – Mei 2020 (1 Tahun)'
    institution_name VARCHAR(255) NOT NULL,          -- e.g. 'RSUD dr. R. Soetrasno Rembang & Puskesmas Sulang'
    description TEXT NOT NULL,
    order_index INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table: Medical Certifications (Formulir Section 3: ACLS, ATLS, FCCS)
CREATE TABLE public.applicant_certifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,                     -- e.g. 'ACLS PERKI', 'ATLS IKABI'
    subtitle VARCHAR(150) NOT NULL,                  -- e.g. 'Adv. Cardiac Life Support'
    issuer VARCHAR(150) NOT NULL,                    -- e.g. 'PERKI', 'Komite Trauma IKABI'
    year_certified INT NOT NULL,                     -- e.g. 2023
    valid_until DATE,
    is_verified BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table: Emergency Contact & Family (Formulir Section 4: Kontak Darurat)
CREATE TABLE public.applicant_emergency_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID UNIQUE NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,                      -- e.g. 'drg. Ratna Kusuma, Sp.KG'
    relationship VARCHAR(100) NOT NULL,              -- e.g. 'Istri'
    phone_number VARCHAR(50) NOT NULL,               -- e.g. '+62 812-3498-7712'
    domicile_address TEXT NOT NULL,                  -- e.g. 'Jl. Candi Prambanan No. 18, Semarang'
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ------------------------------------------------------------------------------
-- 6. UPLOADED DOCUMENTS & PASFOTO DOKTER (unggah_berkas.html)
-- ------------------------------------------------------------------------------

-- Table: Candidate Official Medical Photograph (Pasfoto Dokter Resmi 4x6 Jas Snelli Latar Merah)
CREATE TABLE public.applicant_photos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID UNIQUE NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    storage_path TEXT NOT NULL,                      -- Path in Supabase storage bucket 'candidate-photos'
    photo_url TEXT NOT NULL,
    resolution VARCHAR(50) DEFAULT '1200x1800 px (300 DPI)',
    file_size_kb INT DEFAULT 840,
    has_snelli_suit BOOLEAN DEFAULT true,
    has_red_background BOOLEAN DEFAULT true,
    validation_status public.verification_status NOT NULL DEFAULT 'valid',
    verifier_notes TEXT DEFAULT 'Terverifikasi Sesuai Spesifikasi KKI & Kolegium',
    verified_at TIMESTAMPTZ DEFAULT now(),
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table: Candidate Uploaded Mandatory Documents (The 9 Documents in unggah_berkas.html)
CREATE TABLE public.applicant_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    requirement_id UUID NOT NULL REFERENCES public.document_requirements(id) ON DELETE RESTRICT,
    
    file_name VARCHAR(255) NOT NULL,
    storage_path TEXT NOT NULL,                      -- Path in bucket 'candidate-documents'
    file_url TEXT NOT NULL,
    file_size_kb INT NOT NULL,
    file_format VARCHAR(10) NOT NULL DEFAULT 'PDF',
    
    validation_status public.verification_status NOT NULL DEFAULT 'pending',
    verifier_notes TEXT,                             -- e.g. 'Legalisasi Dekanat & LLDIKTI tervalidasi'
    verifier_id UUID REFERENCES public.profiles(id),
    verified_at TIMESTAMPTZ,
    
    -- Digital Signature BSrE / SatuSehat SDMK metadata
    digital_signature_verified BOOLEAN DEFAULT true,
    digital_signature_provider VARCHAR(100) DEFAULT 'BSrE BSSN & KKI',
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT uq_app_document UNIQUE(application_id, requirement_id)
);

-- ------------------------------------------------------------------------------
-- 7. STAGE PIPELINE TRACKING (index.html & hasil_pengumuman.html)
-- ------------------------------------------------------------------------------

-- Table: Stage-by-Stage Tracking Record per Application
CREATE TABLE public.application_stage_tracking (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    stage_number INT NOT NULL REFERENCES public.selection_stages(stage_number),
    
    status_label VARCHAR(100) NOT NULL,              -- e.g. 'Tahap 4 • Lulus', 'Tahap 5 • Jadwal Aktif'
    title VARCHAR(150) NOT NULL,                     -- e.g. 'Akademik & Kesehatan', 'Wawancara Panelis'
    stage_date DATE,                                 -- e.g. '2025-08-10'
    description TEXT,                                -- e.g. 'TPA 592 & Hasil MMPI/MCU Fit for Clinical.'
    is_completed BOOLEAN NOT NULL DEFAULT false,
    is_current BOOLEAN NOT NULL DEFAULT false,
    order_index INT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT uq_app_stage UNIQUE(application_id, stage_number)
);

-- ------------------------------------------------------------------------------
-- 8. EVALUATION & SCORES RECAPITULATION (Transkrip Evaluasi Masuk Tahap 1, 2, 3)
-- ------------------------------------------------------------------------------

-- Table: Evaluation Components (Materi Evaluasi / Komponen Nilai)
CREATE TABLE public.evaluation_components (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stage_number INT NOT NULL,                       -- 1 (Administrasi), 2 (Akademik & MCU), 3 (Wawancara)
    code VARCHAR(50) UNIQUE NOT NULL,                -- e.g. 'TPA_BAPPENAS', 'CBT_ANESTESI', 'MMPI_2', 'MCU_FISIK'
    name VARCHAR(255) NOT NULL,                      -- e.g. 'Tes Potensi Akademik (TPA Bappenas)'
    sub_description TEXT,                            -- e.g. 'Kemampuan Verbal, Numerik, Logika Spasial'
    weight_percentage NUMERIC(5,2),                  -- e.g. 20.00% (or NULL if mandatory qualitative)
    passing_threshold_label VARCHAR(100) NOT NULL,   -- e.g. '500.00' or 'Tidak Terindikasi Psikopatologi'
    passing_threshold_numeric NUMERIC(6,2),          -- e.g. 500.00
    is_mandatory BOOLEAN NOT NULL DEFAULT true,
    order_index INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table: Candidate Evaluation Scores per Component
CREATE TABLE public.applicant_evaluations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    component_id UUID NOT NULL REFERENCES public.evaluation_components(id) ON DELETE RESTRICT,
    
    score_numeric NUMERIC(6,2),                      -- e.g. 592.50, 84.20
    score_qualitative VARCHAR(100),                  -- e.g. 'Valid & Stabil', 'Fit For Duty'
    result_status VARCHAR(50) NOT NULL,              -- e.g. 'MEMENUHI', 'REKOMENDASI', 'LAIK'
    
    evaluator_notes TEXT,
    evaluator_id UUID REFERENCES public.profiles(id),
    evaluated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT uq_app_evaluation UNIQUE(application_id, component_id)
);

-- ------------------------------------------------------------------------------
-- 9. INTERVIEW SCHEDULE & PANEL COMMITTEE (Tahap 3 Wawancara)
-- ------------------------------------------------------------------------------

-- Table: Candidate Interview Sessions (Jadwal Seleksi Wawancara & Pendalaman Klinis)
CREATE TABLE public.interview_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID UNIQUE NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    
    interview_date DATE NOT NULL,                    -- e.g. '2025-08-18'
    start_time TIME NOT NULL,                        -- e.g. '08:30:00'
    end_time TIME NOT NULL,                          -- e.g. '10:45:00'
    session_label VARCHAR(100) NOT NULL DEFAULT 'Sesi I - Panel A',
    
    -- Lokasi & Ruang Sidang
    room_name VARCHAR(255) NOT NULL DEFAULT 'Ruang Sidang Dekanat Lt. 3',
    building_name VARCHAR(255) NOT NULL DEFAULT 'Gedung FK Universitas Tanjungpura',
    venue_address TEXT NOT NULL DEFAULT 'Jl. Prof. Dr. Hadari Nawawi, Pontianak, Kalimantan Barat',
    
    -- Dewan Penguji
    panel_name VARCHAR(255) NOT NULL DEFAULT 'Panel KATI & Untan',
    panel_description TEXT NOT NULL DEFAULT 'Tim Penguji Kolegium Anestesiologi dan Terapi Intensif Indonesia (KATI) & KPS PPDS Anestesiologi FK UNTAN / RSUD dr. Soedarso',
    
    -- Materi & Ketentuan
    topics_guideline TEXT NOT NULL DEFAULT 'Wawancara Kasus Klinis Anestesi, Tata Laksana Saluran Napas (Airway), Kritis Terapi Intensif (ICU/Resusitasi), dan Komitmen Residensi.',
    dress_code TEXT DEFAULT 'Kemeja Putih Lengan Panjang, Jas Dokter (Snelli), Dasi, & Celana Bahan Gelap.',
    required_items JSONB DEFAULT '["Kartu Peserta Ujian Wawancara 2025", "Portofolio Klinis Mandiri Asli", "Surat Izin Rekomendasi Instansi / Dinas", "Ijazah Dokter & STR Asli dalam Map Folio Biru rangkap 4"]'::jsonb,
    
    syllabus_download_url TEXT,
    exam_card_download_url TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ------------------------------------------------------------------------------
-- 10. OFFICIAL DECREES, ANNOUNCEMENTS & NOTIFICATIONS
-- ------------------------------------------------------------------------------

-- Table: Official Decrees / SK Yudisium Kelulusan Resmi
CREATE TABLE public.decrees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES public.applications(id) ON DELETE CASCADE,
    decree_number VARCHAR(100) NOT NULL,             -- e.g. '892/SK-P2/UNTAN/2025'
    rector_decree_number VARCHAR(100) DEFAULT '4412/UN7/PPDS/2025',
    title VARCHAR(255) NOT NULL,                     -- e.g. 'SURAT KEPUTUSAN HASIL SELEKSI TAHAP 2'
    stage_name VARCHAR(100) NOT NULL DEFAULT 'Tahap 2 (Akademik & Kesehatan)',
    decision_status VARCHAR(100) NOT NULL DEFAULT 'MEMENUHI SYARAT (LULUS)',
    signer_name VARCHAR(255) NOT NULL DEFAULT 'dr. Muhammad Asroruddin, Sp.M',
    signer_title VARCHAR(255) NOT NULL DEFAULT 'Dekan FK UNTAN / Ketua Komisi Seleksi PPDS UNTAN',
    issued_date DATE NOT NULL DEFAULT '2025-08-12',
    verification_hash VARCHAR(100) NOT NULL DEFAULT 'KEMENKES-UNTAN-VERIFIED-2025-892',
    pdf_url TEXT,
    is_published BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table: Portal Announcements & Warta Panitia
CREATE TABLE public.portal_announcements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    period_id UUID REFERENCES public.admission_periods(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author_name VARCHAR(100) DEFAULT 'Sekretariat Panitia Seleksi PPDS UNTAN',
    is_pinned BOOLEAN NOT NULL DEFAULT false,
    publish_date TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table: In-App User Notifications (Header notification bell)
CREATE TABLE public.applicant_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES public.profiles(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT false,
    action_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Table: Helpdesk Contacts
CREATE TABLE public.helpdesk_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,                      -- e.g. 'Helpdesk Seleksi PPDS UNTAN'
    whatsapp_number VARCHAR(50) NOT NULL,            -- e.g. '0812-5678-UNTAN'
    email VARCHAR(100) NOT NULL,                     -- e.g. 'ppds.fk@untan.ac.id'
    office_address TEXT NOT NULL,                    -- e.g. 'Sekretariat Panitia Seleksi PPDS Gedung FK UNTAN Pontianak'
    is_active BOOLEAN NOT NULL DEFAULT true
);

-- ------------------------------------------------------------------------------
-- 11. INDEXES FOR HIGH-PERFORMANCE QUERIES
-- ------------------------------------------------------------------------------
CREATE INDEX idx_profiles_role ON public.profiles(role);
CREATE INDEX idx_profiles_nik ON public.profiles(nik);
CREATE INDEX idx_applications_user_id ON public.applications(user_id);
CREATE INDEX idx_applications_reg_num ON public.applications(registration_number);
CREATE INDEX idx_applications_status ON public.applications(overall_status);
CREATE INDEX idx_applicant_docs_app_id ON public.applicant_documents(application_id);
CREATE INDEX idx_applicant_docs_status ON public.applicant_documents(validation_status);
CREATE INDEX idx_applicant_eval_app_id ON public.applicant_evaluations(application_id);
CREATE INDEX idx_applicant_stages_app_id ON public.application_stage_tracking(application_id);
CREATE INDEX idx_interview_schedules_app_id ON public.interview_schedules(application_id);
CREATE INDEX idx_notifications_user_unread ON public.applicant_notifications(user_id, is_read);

-- ------------------------------------------------------------------------------
-- 12. HELPER FUNCTIONS & TRIGGERS
-- ------------------------------------------------------------------------------

-- Auto-update updated_at timestamp function
CREATE OR REPLACE FUNCTION public.handle_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger updated_at on relevant tables
CREATE TRIGGER trg_profiles_updated_at
    BEFORE UPDATE ON public.profiles
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE TRIGGER trg_admission_periods_updated_at
    BEFORE UPDATE ON public.admission_periods
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE TRIGGER trg_applications_updated_at
    BEFORE UPDATE ON public.applications
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

-- Automatic Registration Number Sequence Generator
CREATE OR REPLACE FUNCTION public.generate_ppds_registration_number(year_str text)
RETURNS text AS $$
DECLARE
    seq_num int;
    new_reg_num text;
BEGIN
    SELECT COALESCE(MAX(SUBSTRING(registration_number FROM '[0-9]+$')::int), 0) + 1
    INTO seq_num
    FROM public.applications
    WHERE registration_number LIKE 'REG-PPDS-' || year_str || '-%';

    new_reg_num := 'REG-PPDS-' || year_str || '-' || LPAD(seq_num::text, 4, '0');
    RETURN new_reg_num;
END;
$$ LANGUAGE plpgsql;

-- Sync Auth Users to Profiles Trigger
CREATE OR REPLACE FUNCTION public.handle_new_auth_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.profiles (
        id,
        role,
        full_name,
        created_at,
        updated_at
    ) VALUES (
        NEW.id,
        'candidate',
        COALESCE(NEW.raw_user_meta_data->>'full_name', 'dr. ' || split_part(NEW.email, '@', 1)),
        now(),
        now()
    ) ON CONFLICT (id) DO NOTHING;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

CREATE OR REPLACE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_new_auth_user();

-- Immutability Guard: Prevent modifications when application is locked
CREATE OR REPLACE FUNCTION public.check_application_lock()
RETURNS TRIGGER AS $$
DECLARE
    app_locked boolean;
BEGIN
    -- Check if parent application is locked
    IF TG_TABLE_NAME = 'applications' THEN
        IF OLD.is_locked = true AND (NEW.is_locked = true) AND (
            OLD.admission_path IS DISTINCT FROM NEW.admission_path OR
            OLD.funding_type IS DISTINCT FROM NEW.funding_type OR
            OLD.period_program_id IS DISTINCT FROM NEW.period_program_id
        ) THEN
            -- Only verifiers/admins can edit locked applications
            IF auth.jwt() ->> 'role' NOT IN ('verifier', 'admin', 'service_role') THEN
                RAISE EXCEPTION 'Formulir pendaftaran telah dikunci dan tidak dapat diubah kembali.';
            END IF;
        END IF;
    ELSE
        SELECT is_locked INTO app_locked FROM public.applications WHERE id = OLD.application_id;
        IF app_locked = true THEN
            IF auth.jwt() ->> 'role' NOT IN ('verifier', 'admin', 'service_role') THEN
                RAISE EXCEPTION 'Berkas pendaftaran telah dikunci untuk verifikasi panitia.';
            END IF;
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_guard_locked_applications
    BEFORE UPDATE ON public.applications
    FOR EACH ROW EXECUTE FUNCTION public.check_application_lock();

CREATE TRIGGER trg_guard_locked_documents
    BEFORE UPDATE OR DELETE ON public.applicant_documents
    FOR EACH ROW EXECUTE FUNCTION public.check_application_lock();

-- ------------------------------------------------------------------------------
-- 13. VIEWS FOR FRONTEND CONSUMPTION
-- ------------------------------------------------------------------------------

-- View 1: Complete Candidate Dashboard Overview (for index.html)
CREATE OR REPLACE VIEW public.v_candidate_dashboard AS
SELECT 
    a.id AS application_id,
    a.registration_number,
    a.overall_status,
    a.current_stage,
    a.is_locked,
    a.rank,
    a.total_cohort_candidates,
    p.id AS user_id,
    p.full_name AS candidate_name,
    p.str_number,
    p.str_valid_until,
    p.idi_number,
    p.idi_branch,
    sp.code AS program_code,
    sp.name AS program_name,
    sp.degree_title AS program_degree,
    sp.faculty AS program_faculty,
    sp.collegium_name,
    pp.quota_seats,
    ap.academic_year,
    ap.semester,
    ap.batch_name,
    (
        SELECT photo_url 
        FROM public.applicant_photos 
        WHERE application_id = a.id 
        LIMIT 1
    ) AS photo_url,
    (
        SELECT jsonb_build_object(
            'interview_date', i.interview_date,
            'start_time', i.start_time,
            'end_time', i.end_time,
            'room_name', i.room_name,
            'building_name', i.building_name,
            'panel_name', i.panel_name
        )
        FROM public.interview_schedules i
        WHERE i.application_id = a.id
        LIMIT 1
    ) AS upcoming_interview
FROM public.applications a
JOIN public.profiles p ON a.user_id = p.id
JOIN public.period_programs pp ON a.period_program_id = pp.id
JOIN public.study_programs sp ON pp.program_id = sp.id
JOIN public.admission_periods ap ON pp.period_id = ap.id;

-- View 2: Document Completeness Progress (for unggah_berkas.html)
CREATE OR REPLACE VIEW public.v_candidate_document_progress AS
SELECT 
    a.id AS application_id,
    a.registration_number,
    COUNT(dr.id) AS total_required,
    COUNT(ad.id) AS total_uploaded,
    COUNT(CASE WHEN ad.validation_status = 'valid' THEN 1 END) AS total_valid,
    COUNT(CASE WHEN ad.validation_status = 'pending' THEN 1 END) AS total_pending,
    COUNT(CASE WHEN ad.validation_status = 'needs_revision' THEN 1 END) AS total_revision_needed,
    ROUND((COUNT(ad.id)::numeric / NULLIF(COUNT(dr.id), 0)) * 100, 1) AS upload_percentage,
    (CASE 
        WHEN COUNT(ad.id) = COUNT(dr.id) AND COUNT(CASE WHEN ad.validation_status = 'valid' THEN 1 END) = COUNT(dr.id) THEN '100% LENGKAP & VALID'
        WHEN COUNT(ad.id) = COUNT(dr.id) THEN '100% TERUNGGAH'
        ELSE 'BELUM LENGKAP'
    END) AS audit_summary_label
FROM public.applications a
CROSS JOIN public.document_requirements dr
LEFT JOIN public.applicant_documents ad 
    ON a.id = ad.application_id AND dr.id = ad.requirement_id
WHERE dr.is_mandatory = true
GROUP BY a.id, a.registration_number;

-- View 3: Complete Evaluation Transcript (for hasil_pengumuman.html)
CREATE OR REPLACE VIEW public.v_candidate_evaluations_recap AS
SELECT 
    a.id AS application_id,
    a.registration_number,
    ec.stage_number,
    ec.code AS component_code,
    ec.name AS component_name,
    ec.sub_description,
    ec.weight_percentage,
    ec.passing_threshold_label,
    ae.score_numeric,
    ae.score_qualitative,
    ae.result_status,
    ae.evaluated_at
FROM public.applications a
JOIN public.applicant_evaluations ae ON a.id = ae.application_id
JOIN public.evaluation_components ec ON ae.component_id = ec.id
ORDER BY ec.stage_number, ec.order_index;

-- ------------------------------------------------------------------------------
-- 14. ROW LEVEL SECURITY (RLS) POLICIES
-- ------------------------------------------------------------------------------
-- Enable RLS on all tables
ALTER TABLE public.admission_periods ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.study_programs ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.teaching_hospitals ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.program_hospital_affiliations ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.period_programs ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.document_requirements ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.selection_stages ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applications ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applicant_academic_histories ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applicant_clinical_experiences ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applicant_certifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applicant_emergency_contacts ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applicant_photos ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applicant_documents ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.application_stage_tracking ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.evaluation_components ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applicant_evaluations ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.interview_schedules ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.decrees ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.portal_announcements ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.applicant_notifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.helpdesk_contacts ENABLE ROW LEVEL SECURITY;

-- Helper security functions
CREATE OR REPLACE FUNCTION public.current_user_role()
RETURNS public.user_role AS $$
    SELECT role FROM public.profiles WHERE id = auth.uid();
$$ LANGUAGE sql STABLE SECURITY DEFINER;

-- 14.1 Public & Master Data Policies (Read for all authenticated or public users)
CREATE POLICY "Public Read for Active Periods" ON public.admission_periods
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Study Programs" ON public.study_programs
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Teaching Hospitals" ON public.teaching_hospitals
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Program Hospital Affiliations" ON public.program_hospital_affiliations
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Period Programs" ON public.period_programs
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Document Requirements" ON public.document_requirements
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Selection Stages" ON public.selection_stages
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Evaluation Components" ON public.evaluation_components
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Announcements" ON public.portal_announcements
    FOR SELECT USING (true);

CREATE POLICY "Public Read for Helpdesk Contacts" ON public.helpdesk_contacts
    FOR SELECT USING (true);

-- 14.2 Profiles Policies
CREATE POLICY "Users can read own profile" ON public.profiles
    FOR SELECT USING (auth.uid() = id);

CREATE POLICY "Staff can view all profiles" ON public.profiles
    FOR SELECT USING (public.current_user_role() IN ('verifier', 'assessor', 'interviewer', 'admin', 'super_admin'));

CREATE POLICY "Users can update own profile" ON public.profiles
    FOR UPDATE USING (auth.uid() = id)
    WITH CHECK (auth.uid() = id);

-- 14.3 Applications Policies
CREATE POLICY "Candidates can read own applications" ON public.applications
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Staff can read all applications" ON public.applications
    FOR SELECT USING (public.current_user_role() IN ('verifier', 'assessor', 'interviewer', 'admin', 'super_admin'));

CREATE POLICY "Candidates can create own application" ON public.applications
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Candidates can update own application when not locked" ON public.applications
    FOR UPDATE USING (auth.uid() = user_id AND is_locked = false)
    WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Admins can update applications anytime" ON public.applications
    FOR UPDATE USING (public.current_user_role() IN ('admin', 'super_admin'));

-- 14.4 Sub-application Data (Academic History, Clinical Experience, Certs, Emergency Contact)
CREATE POLICY "Candidate access own academic history" ON public.applicant_academic_histories
    FOR ALL USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Staff read academic history" ON public.applicant_academic_histories
    FOR SELECT USING (public.current_user_role() IN ('verifier', 'assessor', 'interviewer', 'admin', 'super_admin'));

CREATE POLICY "Candidate access own clinical experiences" ON public.applicant_clinical_experiences
    FOR ALL USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Staff read clinical experiences" ON public.applicant_clinical_experiences
    FOR SELECT USING (public.current_user_role() IN ('verifier', 'assessor', 'interviewer', 'admin', 'super_admin'));

CREATE POLICY "Candidate access own certifications" ON public.applicant_certifications
    FOR ALL USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Staff read certifications" ON public.applicant_certifications
    FOR SELECT USING (public.current_user_role() IN ('verifier', 'assessor', 'interviewer', 'admin', 'super_admin'));

CREATE POLICY "Candidate access own emergency contacts" ON public.applicant_emergency_contacts
    FOR ALL USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Staff read emergency contacts" ON public.applicant_emergency_contacts
    FOR SELECT USING (public.current_user_role() IN ('verifier', 'admin', 'super_admin'));

-- 14.5 Photos and Documents Policies
CREATE POLICY "Candidate access own photos" ON public.applicant_photos
    FOR ALL USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Staff read candidate photos" ON public.applicant_photos
    FOR SELECT USING (public.current_user_role() IN ('verifier', 'assessor', 'interviewer', 'admin', 'super_admin'));

CREATE POLICY "Staff update photo verification status" ON public.applicant_photos
    FOR UPDATE USING (public.current_user_role() IN ('verifier', 'admin', 'super_admin'));

CREATE POLICY "Candidate access own documents" ON public.applicant_documents
    FOR ALL USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Staff read candidate documents" ON public.applicant_documents
    FOR SELECT USING (public.current_user_role() IN ('verifier', 'assessor', 'interviewer', 'admin', 'super_admin'));

CREATE POLICY "Verifiers can update document verification status" ON public.applicant_documents
    FOR UPDATE USING (public.current_user_role() IN ('verifier', 'admin', 'super_admin'));

-- 14.6 Stage Tracking, Evaluations & Interview Policies
CREATE POLICY "Candidate read own stage tracking" ON public.application_stage_tracking
    FOR SELECT USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Staff manage stage tracking" ON public.application_stage_tracking
    FOR ALL USING (public.current_user_role() IN ('verifier', 'assessor', 'admin', 'super_admin'));

CREATE POLICY "Candidate read own published evaluations" ON public.applicant_evaluations
    FOR SELECT USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Assessors and Admins manage evaluations" ON public.applicant_evaluations
    FOR ALL USING (public.current_user_role() IN ('assessor', 'interviewer', 'admin', 'super_admin'));

CREATE POLICY "Candidate read own interview schedule" ON public.interview_schedules
    FOR SELECT USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
    );

CREATE POLICY "Staff manage interview schedule" ON public.interview_schedules
    FOR ALL USING (public.current_user_role() IN ('interviewer', 'admin', 'super_admin'));

CREATE POLICY "Candidate read own decrees" ON public.decrees
    FOR SELECT USING (
        EXISTS (SELECT 1 FROM public.applications a WHERE a.id = application_id AND a.user_id = auth.uid())
        AND is_published = true
    );

CREATE POLICY "Admins manage decrees" ON public.decrees
    FOR ALL USING (public.current_user_role() IN ('admin', 'super_admin'));

CREATE POLICY "Users read own notifications" ON public.applicant_notifications
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users update own notifications read state" ON public.applicant_notifications
    FOR UPDATE USING (auth.uid() = user_id)
    WITH CHECK (auth.uid() = user_id);

-- ------------------------------------------------------------------------------
-- 15. SUPABASE STORAGE BUCKETS & STORAGE RLS POLICIES
-- ------------------------------------------------------------------------------
-- Setting up buckets for candidate photos, candidate documents, and decrees
INSERT INTO storage.buckets (id, name, public, file_size_limit, allowed_mime_types)
VALUES 
    ('candidate-photos', 'candidate-photos', true, 2097152, ARRAY['image/jpeg', 'image/png']),
    ('candidate-documents', 'candidate-documents', false, 3145728, ARRAY['application/pdf']),
    ('official-decrees', 'official-decrees', true, 5242880, ARRAY['application/pdf'])
ON CONFLICT (id) DO UPDATE SET
    file_size_limit = EXCLUDED.file_size_limit,
    allowed_mime_types = EXCLUDED.allowed_mime_types;

-- Storage Policy: Candidate Photos
CREATE POLICY "Candidate photos are publicly viewable"
ON storage.objects FOR SELECT
USING (bucket_id = 'candidate-photos');

CREATE POLICY "Candidates can upload their own photo"
ON storage.objects FOR INSERT
WITH CHECK (
    bucket_id = 'candidate-photos' 
    AND auth.uid()::text = (storage.foldername(name))[1]
);

-- Storage Policy: Candidate Documents (Private PDF, signed or authorized read)
CREATE POLICY "Candidates can read their own documents"
ON storage.objects FOR SELECT
USING (
    bucket_id = 'candidate-documents' 
    AND (
        auth.uid()::text = (storage.foldername(name))[1] 
        OR public.current_user_role() IN ('verifier', 'assessor', 'interviewer', 'admin', 'super_admin')
    )
);

CREATE POLICY "Candidates can upload their own documents"
ON storage.objects FOR INSERT
WITH CHECK (
    bucket_id = 'candidate-documents' 
    AND auth.uid()::text = (storage.foldername(name))[1]
);

CREATE POLICY "Decrees are publicly accessible"
ON storage.objects FOR SELECT
USING (bucket_id = 'official-decrees');
