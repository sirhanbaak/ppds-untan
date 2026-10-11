-- ==============================================================================
-- SUPABASE SEED DATA: PORTAL SELEKSI PPDS FK UNTAN
-- ==============================================================================
-- Populates exact data from mockups:
-- - Periode Ganjil 2025/2026 Gelombang I
-- - Program Studi Spesialis Anestesiologi dan Terapi Intensif (Sp.An-TI)
-- - Afiliasi RS: RSUD Dr. Soedarso, RS UNTAN, RSUD SSM Alkadrie
-- - Auth Users & Calon Residen: dr. Agung Setiawan, S.Ked (PPDS-2025-0892)
-- - Dekan / Verifier: dr. Muhammad Asroruddin, Sp.M
-- - 9 Dokumen Wajib & Pasfoto Resmi Dokter
-- - 6 Stepper Stages & Transkrip Evaluasi Nilai Lengkap
-- - Jadwal Wawancara Panel KATI & Untan (18 Agustus 2025)
-- - SK Yudisium Tahap 2 (No: 892/SK-P2/UNTAN/2025)
-- ==============================================================================

-- ------------------------------------------------------------------------------
-- 1. SELECTION PERIOD & HOSPITALS & STUDY PROGRAM
-- ------------------------------------------------------------------------------
INSERT INTO public.admission_periods (
    id, academic_year, semester, batch_name, status,
    start_date, end_date, cbt_exam_date, cbt_exam_location,
    stage_1_announcement_date, stage_2_announcement_date,
    interview_start_date, final_announcement_date, is_active
) VALUES (
    'a0000000-0000-0000-0000-000000000001',
    '2025/2026', 'Ganjil', 'Gelombang I (Jalur Utama)', 'stage_3_interview',
    '2025-07-01 00:00:00+07', '2025-08-31 23:59:59+07',
    '2025-07-18', 'Pusat Asesmen CBT FK UNTAN Pontianak',
    '2025-07-25 14:00:00+07', '2025-08-12 14:00:00+07',
    '2025-08-18 08:30:00+07', '2025-08-25 15:00:00+07',
    true
) ON CONFLICT (id) DO NOTHING;

INSERT INTO public.study_programs (
    id, code, name, degree_title, level, standard_semesters,
    faculty, collegium_name, description, is_active
) VALUES (
    'b0000000-0000-0000-0000-000000000001',
    'SP-031-ANESTI',
    'Spesialis Anestesiologi dan Terapi Intensif',
    'Sp.An-TI',
    'PPDS-1',
    8,
    'Fakultas Kedokteran Universitas Tanjungpura',
    'Kolegium Anestesiologi dan Terapi Intensif Indonesia (KATI)',
    'Program Pendidikan Dokter Spesialis Anestesiologi dan Terapi Intensif berfokus pada manajemen anestesi perioperatif, resusitasi jalan napas darurat, dan perawatan kritis intensif (ICU/ICCU).',
    true
) ON CONFLICT (id) DO NOTHING;

INSERT INTO public.teaching_hospitals (
    id, name, hospital_type, city, province, description, is_main_hospital
) VALUES 
(
    'c0000000-0000-0000-0000-000000000001',
    'RSUD Dokter Soedarso Pontianak',
    'Rumah Sakit Pendidikan Utama (Tipe A)',
    'Pontianak', 'Kalimantan Barat',
    'Pusat Trauma, ICU, & Bedah Sentral IBS',
    true
),
(
    'c0000000-0000-0000-0000-000000000002',
    'Rumah Sakit Universitas Tanjungpura',
    'RS Pendidikan Satelit / RS UNTAN',
    'Pontianak', 'Kalimantan Barat',
    'Layanan Bedah Terencana & Akademik Kedokteran',
    false
),
(
    'c0000000-0000-0000-0000-000000000003',
    'RSUD Sultan Syarif Mohamad Alkadrie',
    'RS Jejaring Wilayah Kalimantan Barat',
    'Pontianak', 'Kalimantan Barat',
    'Jejaring Pelayanan Bedah Regional',
    false
) ON CONFLICT (id) DO NOTHING;

INSERT INTO public.program_hospital_affiliations (
    program_id, hospital_id, is_main_center, rotation_notes
) VALUES 
('b0000000-0000-0000-0000-000000000001', 'c0000000-0000-0000-0000-000000000001', true, 'Rotasi Bedah Sentral, Trauma, & Perawatan Kritis ICU'),
('b0000000-0000-0000-0000-000000000001', 'c0000000-0000-0000-0000-000000000002', false, 'Rotasi Kamar Operasi Elektif & Pendidikan Klinis'),
('b0000000-0000-0000-0000-000000000001', 'c0000000-0000-0000-0000-000000000003', false, 'Rotasi Pelayanan Rawat Bedah Komunitas')
ON CONFLICT (program_id, hospital_id) DO NOTHING;

INSERT INTO public.period_programs (
    id, period_id, program_id, quota_seats, is_single_choice, is_open
) VALUES (
    'd0000000-0000-0000-0000-000000000001',
    'a0000000-0000-0000-0000-000000000001',
    'b0000000-0000-0000-0000-000000000001',
    2, -- Daya Tampung Resmi: 2 Kursi Residen
    true,
    true
) ON CONFLICT (id) DO NOTHING;

-- ------------------------------------------------------------------------------
-- 2. MASTER: 6 SELECTION STAGES & 9 MANDATORY DOCUMENT REQUIREMENTS
-- ------------------------------------------------------------------------------
INSERT INTO public.selection_stages (stage_number, title, subtitle, description) VALUES
(1, 'Pendaftaran Akun', '12 Juli 2025 • Selesai', 'Formulir lengkap & biodata tervalidasi NPA IDI.'),
(2, 'Verifikasi Berkas', '20 Juli 2025 • Selesai', '8/8 Dokumen lolos kurasi Kemenkes RI.'),
(3, 'Seleksi Administrasi', '25 Juli 2025 • Selesai', 'Skor Memenuhi Syarat & Portofolio Disetujui.'),
(4, 'Akademik & Kesehatan', '10 Agustus 2025 • Lulus', 'TPA 592 & Hasil MMPI/MCU Fit for Clinical.'),
(5, 'Wawancara Panelis', '18 Agu 2025, 08.30 WIB • Jadwal Aktif', 'Ruang Sidang Dekanat Gd. Pascasarjana.'),
(6, 'Pengumuman Akhir', '25 Agustus 2025 • Final', 'Pleno Nasional Kolegium Anestesiologi dan Terapi Intensif Indonesia (KATI).')
ON CONFLICT (stage_number) DO NOTHING;

INSERT INTO public.document_requirements (
    id, code, name, description, allowed_format, max_size_kb, is_mandatory, order_index
) VALUES
('e0000000-0000-0000-0000-000000000001', 'DOC_IJAZAH_TRANSKRIP', 'Ijazah Dokter & Transkrip Akademik Terlegalisir', 'Ijazah Sarjana Kedokteran, Profesi Dokter, dan Transkrip Kumulatif.', 'PDF', 2048, true, 1),
('e0000000-0000-0000-0000-000000000002', 'DOC_STR_KKI', 'Surat Tanda Registrasi (STR) Dokter Aktif dari KKI', 'STR Seumur Hidup atau aktif teregistrasi resmi KKI.', 'PDF', 2048, true, 2),
('e0000000-0000-0000-0000-000000000003', 'DOC_SIP_INTERNSHIP', 'Surat Izin Praktik (SIP) Terakhir / Bukti Selesai Internship (STSI)', 'Surat Tanda Selesai Internship terbitan KIDI Kemenkes RI.', 'PDF', 2048, true, 3),
('e0000000-0000-0000-0000-000000000004', 'DOC_UKMPPD', 'Sertifikat Kelulusan Uji Kompetensi (UKMPPD)', 'Sertifikat kompetensi dokter umum terbitan Panitia UKMPPD Nasional.', 'PDF', 2048, true, 4),
('e0000000-0000-0000-0000-000000000005', 'DOC_ACLS_ATLS', 'Sertifikat ACLS / ATLS Masih Berlaku', 'Sertifikasi Advanced Cardiac/Trauma Life Support PERKI / IKABI.', 'PDF', 2048, true, 5),
('e0000000-0000-0000-0000-000000000006', 'DOC_SEHAT_FISIK', 'Surat Keterangan Sehat Fisik RS Tipe A/B', 'Hasil pemeriksaan fisik MCU lengkap Calon Residen RS Rujukan.', 'PDF', 2048, true, 6),
('e0000000-0000-0000-0000-000000000007', 'DOC_NARKOBA_BUTA_WARNA', 'Surat Bebas Narkoba (6 Parameter) & Bebas Buta Warna', 'Uji toksikologi 6 parameter dan uji Ishihara 38 plates normal.', 'PDF', 2048, true, 7),
('e0000000-0000-0000-0000-000000000008', 'DOC_REKOMENDASI_IDI', 'Rekomendasi IDI Cabang & 2 Dokter Spesialis Senior', 'Surat pengantar IDI cabang serta form rekomendasi rahasia spesialis.', 'PDF', 2048, true, 8),
('e0000000-0000-0000-0000-000000000009', 'DOC_TOEFL_IELTS', 'Sertifikat Skor Bahasa Inggris (TOEFL ITP / IELTS)', 'ETS Official Institutional Testing Program (min. 475-500).', 'PDF', 2048, true, 9)
ON CONFLICT (id) DO NOTHING;

-- ------------------------------------------------------------------------------
-- 3. MASTER: EVALUATION COMPONENTS (Tahap 1, 2, 3)
-- ------------------------------------------------------------------------------
INSERT INTO public.evaluation_components (
    id, stage_number, code, name, sub_description, weight_percentage,
    passing_threshold_label, passing_threshold_numeric, is_mandatory, order_index
) VALUES
(
    'f0000000-0000-0000-0000-000000000001', 1,
    'ADMIN_TOTAL', 'Kelengkapan Berkas & Legalitas Administrasi',
    'IPK Sarjana Kedokteran & Profesi, Validitas KKI/IDI, Pengalaman Klinis',
    100.00, '≥ 75.00', 75.00, true, 1
),
(
    'f0000000-0000-0000-0000-000000000002', 2,
    'TPA_BAPPENAS', 'Tes Potensi Akademik (TPA Bappenas)',
    'Kemampuan Verbal, Numerik, Logika Spasial',
    20.00, '500.00', 500.00, true, 2
),
(
    'f0000000-0000-0000-0000-000000000003', 2,
    'CBT_ANESTESI', 'CBT Pengetahuan Ilmu Anestesiologi, Fisiologi, Farmakologi Klinik & Terapi Intensif',
    'Ujian tulis spesifik kurikulum dasar anestesi & resusitasi kritis',
    30.00, '70.00', 70.00, true, 3
),
(
    'f0000000-0000-0000-0000-000000000004', 2,
    'MMPI_2', 'Tes Psikometri Klinis & MMPI-2',
    'Profil stabilitas emosional & adaptabilitas residensi',
    NULL, 'Tidak Terindikasi Psikopatologi', NULL, true, 4
),
(
    'f0000000-0000-0000-0000-000000000005', 2,
    'MCU_FISIK', 'Pemeriksaan Fisik Bebas Narkoba & Kesehatan Paru/Mata',
    'MCU RS Rujukan Nasional Dr. Kariadi',
    NULL, 'Bebas Hambatan Fisik', NULL, true, 5
),
(
    'f0000000-0000-0000-0000-000000000006', 3,
    'WAWANCARA_KLINIS', 'Wawancara Panelis, Portofolio & Diskusi Kasus Klinis',
    'Kasus Anestesi, Difficult Airway, Syok Sepsis ICU, & Komitmen Residensi',
    50.00, '≥ 75.00', 75.00, true, 6
)
ON CONFLICT (id) DO NOTHING;

-- ------------------------------------------------------------------------------
-- 4. SUPABASE AUTH USERS & PROFILES
-- ------------------------------------------------------------------------------
-- Create the auth users inside auth.users so foreign keys on public.profiles succeed
DO $$
BEGIN
    -- 4.1 Mock Candidate User: dr. Agung Setiawan, S.Ked
    IF NOT EXISTS (
        SELECT 1 FROM auth.users 
        WHERE id = '11111111-1111-1111-1111-111111111111' 
           OR email = 'agung.setiawan@dokter.untan.ac.id'
    ) THEN
        INSERT INTO auth.users (
            instance_id,
            id,
            aud,
            role,
            email,
            encrypted_password,
            email_confirmed_at,
            raw_app_meta_data,
            raw_user_meta_data,
            created_at,
            updated_at,
            confirmation_token,
            email_change,
            email_change_token_new,
            recovery_token
        ) VALUES (
            '00000000-0000-0000-0000-000000000000',
            '11111111-1111-1111-1111-111111111111',
            'authenticated',
            'authenticated',
            'agung.setiawan@dokter.untan.ac.id',
            '$2a$10$vI8aWBnW3fID.ZQ4/zo1G.q1PHdaYPeqlKviuxmJe49r99b0c79yK', -- 'Password123!'
            now(),
            '{"provider":"email","providers":["email"]}'::jsonb,
            '{"full_name":"dr. Agung Setiawan, S.Ked"}'::jsonb,
            now(),
            now(),
            '', '', '', ''
        );
    END IF;

    -- 4.2 Mock Admin User: dr. Muhammad Asroruddin, Sp.M (Dekan FK UNTAN)
    IF NOT EXISTS (
        SELECT 1 FROM auth.users 
        WHERE id = '22222222-2222-2222-2222-222222222222' 
           OR email = 'dekan.fk@untan.ac.id'
    ) THEN
        INSERT INTO auth.users (
            instance_id,
            id,
            aud,
            role,
            email,
            encrypted_password,
            email_confirmed_at,
            raw_app_meta_data,
            raw_user_meta_data,
            created_at,
            updated_at,
            confirmation_token,
            email_change,
            email_change_token_new,
            recovery_token
        ) VALUES (
            '00000000-0000-0000-0000-000000000000',
            '22222222-2222-2222-2222-222222222222',
            'authenticated',
            'authenticated',
            'dekan.fk@untan.ac.id',
            '$2a$10$vI8aWBnW3fID.ZQ4/zo1G.q1PHdaYPeqlKviuxmJe49r99b0c79yK', -- 'Password123!'
            now(),
            '{"provider":"email","providers":["email"]}'::jsonb,
            '{"full_name":"dr. Muhammad Asroruddin, Sp.M"}'::jsonb,
            now(),
            now(),
            '', '', '', ''
        );
    END IF;

    -- 4.3 Populate auth.identities safely if table exists
    IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'auth' AND table_name = 'identities') THEN
        INSERT INTO auth.identities (
            id, user_id, identity_data, provider, provider_id, last_sign_in_at, created_at, updated_at
        ) VALUES 
        (
            '11111111-1111-1111-1111-111111111111',
            '11111111-1111-1111-1111-111111111111',
            '{"sub":"11111111-1111-1111-1111-111111111111","email":"agung.setiawan@dokter.untan.ac.id"}'::jsonb,
            'email',
            '11111111-1111-1111-1111-111111111111',
            now(), now(), now()
        ),
        (
            '22222222-2222-2222-2222-222222222222',
            '22222222-2222-2222-2222-222222222222',
            '{"sub":"22222222-2222-2222-2222-222222222222","email":"dekan.fk@untan.ac.id"}'::jsonb,
            'email',
            '22222222-2222-2222-2222-222222222222',
            now(), now(), now()
        ) ON CONFLICT (id) DO NOTHING;
    END IF;
EXCEPTION WHEN OTHERS THEN
    -- Ignore any version differences on auth.identities
    NULL;
END $$;

-- 4.4 Populate / Enrich Profiles
INSERT INTO public.profiles (
    id, role, full_name, nik, place_of_birth, date_of_birth,
    gender, blood_group, phone_number, address,
    str_number, str_valid_until, str_is_lifetime, str_is_verified,
    idi_number, idi_branch, idi_is_verified, avatar_url
) VALUES (
    '11111111-1111-1111-1111-111111111111',
    'candidate',
    'dr. Agung Setiawan, S.Ked',
    '3374021405940003',
    'Semarang',
    '1994-05-14',
    'laki-laki',
    'O+',
    '+62 812-3498-7712',
    'Jl. Candi Prambanan No. 18, Candisari, Semarang',
    '33.1.1.100.2.20.187293',
    '2028-05-14',
    false,
    true,
    'NPA-142089',
    'Kota Semarang, Jawa Tengah',
    true,
    'https://lh3.googleusercontent.com/aida-public/AB6AXuB3iXfipsSIXgK7xUIK4nU8d_47ZekCMHCWjMonjVpa82n4E44GSPl3sseiCD_PRAv24auNSSHOMYKFPbrYGMmhF4ewg1ycxW4bpUlShdDS1DEpNix1Z1AoZQVDR7DHlfylnvjOM2ebMYkWAGoaPfKBpeVgHkk9xTFxbnFI0gMsklk17lXi_V-AZyHfFMiKvgSsI7UCep74_sXuVApxfvNE_A10yA0go4XPzwzV2Rt8CUv1JidvJLRM'
) ON CONFLICT (id) DO UPDATE SET
    role = EXCLUDED.role,
    full_name = EXCLUDED.full_name,
    nik = EXCLUDED.nik,
    place_of_birth = EXCLUDED.place_of_birth,
    date_of_birth = EXCLUDED.date_of_birth,
    gender = EXCLUDED.gender,
    blood_group = EXCLUDED.blood_group,
    phone_number = EXCLUDED.phone_number,
    address = EXCLUDED.address,
    str_number = EXCLUDED.str_number,
    str_valid_until = EXCLUDED.str_valid_until,
    str_is_lifetime = EXCLUDED.str_is_lifetime,
    str_is_verified = EXCLUDED.str_is_verified,
    idi_number = EXCLUDED.idi_number,
    idi_branch = EXCLUDED.idi_branch,
    idi_is_verified = EXCLUDED.idi_is_verified,
    avatar_url = EXCLUDED.avatar_url;

-- Verifier Profile: dr. Muhammad Asroruddin, Sp.M (Dekan FK UNTAN)
INSERT INTO public.profiles (
    id, role, full_name, nik, phone_number, address
) VALUES (
    '22222222-2222-2222-2222-222222222222',
    'admin',
    'dr. Muhammad Asroruddin, Sp.M',
    '6171011508750001',
    '+62 812-5678-9000',
    'Dekanat Fakultas Kedokteran Universitas Tanjungpura, Pontianak'
) ON CONFLICT (id) DO UPDATE SET
    role = EXCLUDED.role,
    full_name = EXCLUDED.full_name,
    nik = EXCLUDED.nik,
    phone_number = EXCLUDED.phone_number,
    address = EXCLUDED.address;

-- ------------------------------------------------------------------------------
-- 5. CANDIDATE APPLICATION & SUB-SECTIONS
-- ------------------------------------------------------------------------------
INSERT INTO public.applications (
    id, registration_number, user_id, period_program_id,
    admission_path, funding_type, funding_scheme_name, funding_category,
    funding_document_status, backup_funding_source,
    current_stage, overall_status, is_locked, locked_at,
    rank, total_cohort_candidates
) VALUES (
    '33333333-3333-3333-3333-333333333333',
    'REG-PPDS-2025-0892',
    '11111111-1111-1111-1111-111111111111',
    'd0000000-0000-0000-0000-000000000001',
    'reguler_nasional',
    'lpdp',
    'Beasiswa LPDP Targeted Dokter Spesialis',
    'Dokter Spesialis Prioritas Kemenkes RI',
    'Letter of Sponsorship (LoS) Siap',
    'Mandiri / Rekening Terverifikasi Bank Himbara',
    5, -- Active at Stage 5 (Wawancara Panelis)
    'passed_stage_2',
    true,
    '2025-07-12 11:30:00+07',
    4,
    86
) ON CONFLICT (id) DO NOTHING;

-- Academic Background
INSERT INTO public.applicant_academic_histories (
    application_id, almamater_name, graduation_year,
    gpa_bachelor, gpa_profession, predicate
) VALUES (
    '33333333-3333-3333-3333-333333333333',
    'Fakultas Kedokteran Universitas Airlangga (UNAIR) Surabaya',
    2019,
    3.65,
    3.78,
    'Predikat Sangat Memuaskan'
) ON CONFLICT (application_id) DO NOTHING;

-- Clinical Experiences
INSERT INTO public.applicant_clinical_experiences (
    id, application_id, experience_type, title, duration_label,
    institution_name, description, order_index
) VALUES 
(
    '70000000-0000-0000-0000-000000000001',
    '33333333-3333-3333-3333-333333333333',
    'internship',
    'Program Internship Dokter Indonesia (PIDI)',
    'Mei 2019 – Mei 2020 (1 Tahun)',
    'RSUD dr. R. Soetrasno Rembang & Puskesmas Sulang',
    'Telah menyelesaikan rotasi wajib Stase Rawat Jalan, Ruang Rawat Inap Penyakit Dalam/Anak/Bedah, Instalasi Gawat Darurat (IGD), serta Pelayanan Kesehatan Masyarakat di Puskesmas Sulang. Menyelesaikan Laporan Kasus Komprehensif dengan predikat Memuaskan dari KIDI Provinsi Jawa Tengah.',
    1
),
(
    '70000000-0000-0000-0000-000000000002',
    '33333333-3333-3333-3333-333333333333',
    'clinical_job',
    'Dokter Jaga Instalasi Gawat Darurat (IGD)',
    'Juni 2020 – Sekarang (4 Tahun Masa Kerja)',
    'Rumah Sakit Bhakti Husada, Semarang',
    'Bertanggung jawab penuh atas triase pasien, tatalaksana awal sindrom koroner akut (ACS), henti jantung resusitasi kardiopulmoner tingkat lanjut, stabilisasi syok kardiogenik & sepsis, serta manajemen transport rujukan pasien kritis ICCU/ICU.',
    2
) ON CONFLICT (id) DO NOTHING;

-- Certifications
INSERT INTO public.applicant_certifications (
    id, application_id, title, subtitle, issuer, year_certified, valid_until, is_verified
) VALUES 
('71000000-0000-0000-0000-000000000001', '33333333-3333-3333-3333-333333333333', 'ACLS PERKI', 'Adv. Cardiac Life Support', 'PERKI', 2023, '2026-11-20', true),
('71000000-0000-0000-0000-000000000002', '33333333-3333-3333-3333-333333333333', 'ATLS IKABI', 'Adv. Trauma Life Support', 'Komite Trauma IKABI', 2022, '2026-08-15', true),
('71000000-0000-0000-0000-000000000003', '33333333-3333-3333-3333-333333333333', 'FCCS & Airway Care', 'Crit. Care & Adv. Airway Mgmt', 'PERDATIN & SCCM', 2023, '2027-05-10', true)
ON CONFLICT (id) DO NOTHING;

-- Emergency Contact
INSERT INTO public.applicant_emergency_contacts (
    application_id, name, relationship, phone_number, domicile_address
) VALUES (
    '33333333-3333-3333-3333-333333333333',
    'drg. Ratna Kusuma, Sp.KG',
    'Istri',
    '+62 812-3498-7712',
    'Jl. Candi Prambanan No. 18, Semarang'
) ON CONFLICT (application_id) DO NOTHING;

-- ------------------------------------------------------------------------------
-- 6. PASFOTO RESMI & 9 UPLOADED DOCUMENTS
-- ------------------------------------------------------------------------------
INSERT INTO public.applicant_photos (
    application_id, storage_path, photo_url, resolution, file_size_kb,
    has_snelli_suit, has_red_background, validation_status, verifier_notes
) VALUES (
    '33333333-3333-3333-3333-333333333333',
    '33333333-3333-3333-3333-333333333333/pasfoto_resmi_dr_agung.jpg',
    'https://lh3.googleusercontent.com/aida-public/AB6AXuB3iXfipsSIXgK7xUIK4nU8d_47ZekCMHCWjMonjVpa82n4E44GSPl3sseiCD_PRAv24auNSSHOMYKFPbrYGMmhF4ewg1ycxW4bpUlShdDS1DEpNix1Z1AoZQVDR7DHlfylnvjOM2ebMYkWAGoaPfKBpeVgHkk9xTFxbnFI0gMsklk17lXi_V-AZyHfFMiKvgSsI7UCep74_sXuVApxfvNE_A10yA0go4XPzwzV2Rt8CUv1JidvJLRM',
    '1200x1800 px (300 DPI)',
    840,
    true,
    true,
    'valid',
    'Terverifikasi Sesuai Spesifikasi KKI & Kolegium'
) ON CONFLICT (application_id) DO NOTHING;

-- The 9 Documents in Table of unggah_berkas.html
INSERT INTO public.applicant_documents (
    application_id, requirement_id, file_name, storage_path, file_url,
    file_size_kb, file_format, validation_status, verifier_notes, verified_at
) VALUES 
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000001',
    'Ijazah_Dokter_Transkrip_UNAIR_dr_Agung.pdf',
    '33333333-3333-3333-3333-333333333333/ijazah_transkrip.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/ijazah_transkrip.pdf',
    1843, 'PDF', 'valid', 'Legalisasi Dekanat & LLDIKTI tervalidasi', '2025-07-20 10:15:00+07'
),
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000002',
    'STR_Dokter_Aktif_KKI_dr_Agung.pdf',
    '33333333-3333-3333-3333-333333333333/str_kki.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/str_kki.pdf',
    640, 'PDF', 'valid', 'Sinkronisasi SatuSehat SDMK otomatis lolos', '2025-07-20 10:18:00+07'
),
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000003',
    'STSI_KIDI_SIP_Terakhir_dr_Agung.pdf',
    '33333333-3333-3333-3333-333333333333/stsi_kidi.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/stsi_kidi.pdf',
    890, 'PDF', 'valid', 'Masa kerja klinis RSUD Tarakan tervalidasi 12 bulan', '2025-07-20 10:20:00+07'
),
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000004',
    'Sertifikat_UKMPPD_Nasional_dr_Agung.pdf',
    '33333333-3333-3333-3333-333333333333/ukmppd.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/ukmppd.pdf',
    520, 'PDF', 'valid', 'Hasil CBT: 88.50 • OSCE: Pass', '2025-07-20 10:22:00+07'
),
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000005',
    'Sertifikat_ACLS_PERKI_ATLS_IKABI.pdf',
    '33333333-3333-3333-3333-333333333333/acls_atls.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/acls_atls.pdf',
    1126, 'PDF', 'valid', 'Masa berlaku s/d November 2027', '2025-07-20 10:25:00+07'
),
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000006',
    'Surat_Keterangan_Sehat_Fisik_RSCM.pdf',
    '33333333-3333-3333-3333-333333333333/mcu_sehat_fisik.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/mcu_sehat_fisik.pdf',
    750, 'PDF', 'valid', 'Ditandatangani Dokter Penguji Spesialis Okupasi', '2025-07-20 10:28:00+07'
),
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000007',
    'Bebas_Narkoba_6_Param_Bebas_Buta_Warna.pdf',
    '33333333-3333-3333-3333-333333333333/narkoba_ishihara.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/narkoba_ishihara.pdf',
    620, 'PDF', 'valid', 'Uji Ishihara 38 Plates: Normal • Narkoba Negatif', '2025-07-20 10:30:00+07'
),
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000008',
    'Rekomendasi_IDI_Cabang_Spesialis_Senior.pdf',
    '33333333-3333-3333-3333-333333333333/rekomendasi_idi.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/rekomendasi_idi.pdf',
    1433, 'PDF', 'valid', 'SKP IDI Aktif • Lembar rekomendasi tertutup lengkap', '2025-07-20 10:32:00+07'
),
(
    '33333333-3333-3333-3333-333333333333',
    'e0000000-0000-0000-0000-000000000009',
    'Sertifikat_TOEFL_ITP_Official_Score.pdf',
    '33333333-3333-3333-3333-333333333333/toefl_itp.pdf',
    'https://storage.supabase.co/v1/object/public/candidate-documents/toefl_itp.pdf',
    430, 'PDF', 'valid', 'Skor 567 (Standar Minimal 500) • Lembaga ETS Terakreditasi', '2025-07-20 10:35:00+07'
) ON CONFLICT (application_id, requirement_id) DO NOTHING;

-- ------------------------------------------------------------------------------
-- 7. APPLICATION STAGE TRACKING PIPELINE (Stepper Stages)
-- ------------------------------------------------------------------------------
INSERT INTO public.application_stage_tracking (
    application_id, stage_number, status_label, title, stage_date,
    description, is_completed, is_current, order_index
) VALUES 
(
    '33333333-3333-3333-3333-333333333333', 1,
    'Tahap 1 • Selesai', 'Pendaftaran Akun & Formulir', '2025-07-12',
    'Formulir lengkap & biodata tervalidasi NPA IDI.', true, false, 1
),
(
    '33333333-3333-3333-3333-333333333333', 2,
    'Tahap 2 • Selesai', 'Verifikasi Berkas Administrasi', '2025-07-20',
    '9/9 Dokumen lolos kurasi Kemenkes RI & KKI.', true, false, 2
),
(
    '33333333-3333-3333-3333-333333333333', 3,
    'Tahap 3 • Selesai', 'Seleksi Administrasi', '2025-07-25',
    'Skor Memenuhi Syarat & Portofolio Disetujui (Nilai 94.50).', true, false, 3
),
(
    '33333333-3333-3333-3333-333333333333', 4,
    'Tahap 4 • Lulus', 'Akademik & Pemeriksaan Kesehatan', '2025-08-10',
    'TPA 592.50 & Hasil MMPI/MCU Fit for Clinical Practice.', true, false, 4
),
(
    '33333333-3333-3333-3333-333333333333', 5,
    'Tahap 5 • Jadwal Aktif', 'Wawancara Panelis & Uji Komite Kolegium', '2025-08-18',
    'Pukul 08.30 WIB - Ruang Sidang Dekanat Lt. 3 Gd. Pascasarjana FK UNTAN.', false, true, 5
),
(
    '33333333-3333-3333-3333-333333333333', 6,
    'Tahap 6 • Final', 'Pengumuman Akhir Penetapan Residen', '2025-08-25',
    'Pleno Nasional Kolegium Anestesiologi dan Terapi Intensif Indonesia (KATI).', false, false, 6
) ON CONFLICT (application_id, stage_number) DO NOTHING;

-- ------------------------------------------------------------------------------
-- 8. EVALUATION SCORES (Tahap 1 Administrasi & Tahap 2 CBT/MCU)
-- ------------------------------------------------------------------------------
INSERT INTO public.applicant_evaluations (
    application_id, component_id, score_numeric, score_qualitative,
    result_status, evaluator_notes
) VALUES 
(
    '33333333-3333-3333-3333-333333333333',
    'f0000000-0000-0000-0000-000000000001',
    94.50, 'Lengkap & Sah',
    'LULUS',
    'Verifikasi berkas komprehensif 100% valid memenuhi syarat administrasi Kemenkes.'
),
(
    '33333333-3333-3333-3333-333333333333',
    'f0000000-0000-0000-0000-000000000002',
    592.50, NULL,
    'MEMENUHI',
    'Nilai TPA Bappenas resmi melampaui batas ambang standar 500.00.'
),
(
    '33333333-3333-3333-3333-333333333333',
    'f0000000-0000-0000-0000-000000000003',
    84.20, 'Sangat Baik',
    'MEMENUHI',
    'Ujian tulis CBT Anestesiologi, fisiologi dan resusitasi kritis melampaui passing 70.00.'
),
(
    '33333333-3333-3333-3333-333333333333',
    'f0000000-0000-0000-0000-000000000004',
    NULL, 'Valid & Stabil',
    'REKOMENDASI',
    'Hasil uji MMPI-2 profil kepribadian stabil, resiliensi tekanan baik, tidak ada psikopatologi.'
),
(
    '33333333-3333-3333-3333-333333333333',
    'f0000000-0000-0000-0000-000000000005',
    NULL, 'Fit For Duty',
    'LAIK',
    'MCU RSUP Dr. Kariadi: Bebas narkoba 6 parameter, bebas buta warna, laik tugas klinis ICU.'
) ON CONFLICT (application_id, component_id) DO NOTHING;

-- ------------------------------------------------------------------------------
-- 9. INTERVIEW SCHEDULE & COMMITTEE
-- ------------------------------------------------------------------------------
INSERT INTO public.interview_schedules (
    application_id, interview_date, start_time, end_time, session_label,
    room_name, building_name, venue_address,
    panel_name, panel_description, topics_guideline, dress_code,
    syllabus_download_url, exam_card_download_url
) VALUES (
    '33333333-3333-3333-3333-333333333333',
    '2025-08-18', '08:30:00', '10:45:00', 'Sesi I - Panel A',
    'Ruang Sidang Dekanat Lt. 3',
    'Gedung FK Universitas Tanjungpura',
    'Jl. Prof. Dr. Hadari Nawawi, Pontianak, Kalimantan Barat',
    'Panel KATI & Untan',
    'Tim Penguji Kolegium Anestesiologi dan Terapi Intensif Indonesia (KATI) & KPS PPDS Anestesiologi FK UNTAN / RSUD dr. Soedarso',
    'Wawancara Kasus Klinis Anestesi, Tata Laksana Saluran Napas (Airway), Kritis Terapi Intensif (ICU/Resusitasi), dan Komitmen Residensi.',
    'Kemeja Putih Lengan Panjang, Jas Dokter (Snelli), Dasi, & Celana Bahan Gelap.',
    'https://ppds.untan.ac.id/downloads/silabus_wawancara_kati_2025.pdf',
    'https://ppds.untan.ac.id/downloads/kartu_peserta_wawancara_0892.pdf'
) ON CONFLICT (application_id) DO NOTHING;

-- ------------------------------------------------------------------------------
-- 10. OFFICIAL DECREE (SK YUDISIUM TAHAP 2), WARTA & NOTIFICATIONS
-- ------------------------------------------------------------------------------
INSERT INTO public.decrees (
    id, application_id, decree_number, rector_decree_number, title, stage_name,
    decision_status, signer_name, signer_title, issued_date,
    verification_hash, pdf_url, is_published
) VALUES (
    '88888888-8888-8888-8888-888888888888',
    '33333333-3333-3333-3333-333333333333',
    '892/SK-P2/UNTAN/2025',
    '4412/UN7/PPDS/2025',
    'SURAT KEPUTUSAN HASIL SELEKSI TAHAP 2',
    'Tahap 2 (Akademik & Kesehatan)',
    'MEMENUHI SYARAT (LULUS)',
    'dr. Muhammad Asroruddin, Sp.M',
    'Dekan FK UNTAN / Ketua Komisi Seleksi PPDS UNTAN',
    '2025-08-12',
    'KEMENKES-UNTAN-VERIFIED-2025-892',
    'https://ppds.untan.ac.id/sk/892_SK_P2_UNTAN_2025.pdf',
    true
) ON CONFLICT (id) DO NOTHING;

INSERT INTO public.portal_announcements (
    id, period_id, title, content, is_pinned, publish_date
) VALUES 
(
    '90000000-0000-0000-0000-000000000001',
    'a0000000-0000-0000-0000-000000000001',
    'Penerbitan Jadwal Wawancara Resmi Periode Ganjil 2025/2026',
    'Jadwal panel spesifik per kelompok telah dikirim melalui email peserta dan tampil di dashboard portal masing-masing kandidat.',
    true, '2025-08-11 14:20:00+07'
),
(
    '90000000-0000-0000-0000-000000000002',
    'a0000000-0000-0000-0000-000000000001',
    'Hasil Rekapitulasi MMPI-2 & Tes Bebas Narkoba',
    'Seluruh berkas kesehatan telah diunggah dan disinkronkan ke sistem database bersama RS Pendidikan & FK UNTAN.',
    false, '2025-08-10 17:00:00+07'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO public.applicant_notifications (
    id, user_id, title, body, is_read, action_url
) VALUES 
(
    '91000000-0000-0000-0000-000000000001',
    '11111111-1111-1111-1111-111111111111',
    'Jadwal Seleksi Wawancara Terbit',
    'Anda dijadwalkan mengikuti Wawancara Panelis pada Senin, 18 Agustus 2025 pukul 08.30 WIB di Ruang Sidang Dekanat Lt. 3.',
    false,
    '/hasil_pengumuman.html#tahap-3-detail'
),
(
    '91000000-0000-0000-0000-000000000002',
    '11111111-1111-1111-1111-111111111111',
    'SK Yudisium Tahap 2 Telah Diterbitkan',
    'Selamat! Anda dinyatakan LULUS Tahap 2 dan berhak mengikuti seleksi Tahap 3 (Wawancara).',
    true,
    '/hasil_pengumuman.html#unduh-dokumen'
) ON CONFLICT (id) DO NOTHING;

INSERT INTO public.helpdesk_contacts (
    id, name, whatsapp_number, email, office_address, is_active
) VALUES (
    '92000000-0000-0000-0000-000000000001',
    'Helpdesk Seleksi PPDS UNTAN',
    '0812-5678-UNTAN',
    'ppds.fk@untan.ac.id',
    'Sekretariat Panitia Seleksi PPDS Gedung FK UNTAN Pontianak, Jl. Prof. Dr. Hadari Nawawi, Pontianak, Kalimantan Barat',
    true
) ON CONFLICT (id) DO NOTHING;
