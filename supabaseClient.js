// ==============================================================================
// SUPABASE CLIENT SERVICE - PORTAL SELEKSI PPDS FK UNTAN
// ==============================================================================
// Menyediakan fungsi terpadu untuk berinteraksi dengan database dan Supabase Storage
// ==============================================================================

(function () {
  const config = window.SUPABASE_CONFIG || {};

  if (!config.url || config.url.includes("YOUR_PROJECT_REF") || !config.anonKey || config.anonKey.includes("YOUR_SUPABASE_ANON_KEY")) {
    console.warn("[PPDS Supabase] Kredensial belum disetel pada config.js. Menggunakan mode demonstrasi/fallback.");
  }

  // Inisialisasi Supabase Client dari Global CDN supabase
  const supabaseClient = (window.supabase && config.url && config.anonKey && !config.url.includes("YOUR_PROJECT_REF"))
    ? window.supabase.createClient(config.url, config.anonKey)
    : null;

  window.PPDS_API = {
    client: supabaseClient,

    // 1. Memeriksa status koneksi
    isConnected: function () {
      return supabaseClient !== null;
    },

    // 2. Mengambil data dashboard kandidat (index.html)
    getDashboardData: async function (registrationNumber = 'REG-PPDS-2025-0892') {
      if (!supabaseClient) return null;
      try {
        const { data, error } = await supabaseClient
          .from('v_candidate_dashboard')
          .select('*')
          .eq('registration_number', registrationNumber)
          .single();

        if (error) throw error;
        return data;
      } catch (err) {
        console.error('[PPDS Supabase] Gagal mengambil dashboard:', err.message);
        return null;
      }
    },

    // 3. Mengambil pelacak tahapan seleksi (Stepper)
    getStageTracking: async function (applicationId = '33333333-3333-3333-3333-333333333333') {
      if (!supabaseClient) return [];
      try {
        const { data, error } = await supabaseClient
          .from('application_stage_tracking')
          .select('*')
          .eq('application_id', applicationId)
          .order('order_index', { ascending: true });

        if (error) throw error;
        return data || [];
      } catch (err) {
        console.error('[PPDS Supabase] Gagal mengambil stage tracking:', err.message);
        return [];
      }
    },

    // 4. Mengambil transkrip evaluasi nilai (index.html & hasil_pengumuman.html)
    getEvaluationsRecap: async function (applicationId = '33333333-3333-3333-3333-333333333333') {
      if (!supabaseClient) return [];
      try {
        const { data, error } = await supabaseClient
          .from('v_candidate_evaluations_recap')
          .select('*')
          .eq('application_id', applicationId);

        if (error) throw error;
        return data || [];
      } catch (err) {
        console.error('[PPDS Supabase] Gagal mengambil evaluasi nilai:', err.message);
        return [];
      }
    },

    // 5. Mengambil daftar 9 dokumen dan status verifikasinya (unggah_berkas.html)
    getDocumentsList: async function (applicationId = '33333333-3333-3333-3333-333333333333') {
      if (!supabaseClient) return [];
      try {
        const { data, error } = await supabaseClient
          .from('document_requirements')
          .select(`
            id, code, name, description, allowed_format, max_size_kb, is_mandatory, order_index,
            applicant_documents (
              id, file_name, file_url, file_size_kb, validation_status, verifier_notes, verified_at, uploaded_at
            )
          `)
          .order('order_index', { ascending: true });

        if (error) throw error;
        return data || [];
      } catch (err) {
        console.error('[PPDS Supabase] Gagal mengambil daftar dokumen:', err.message);
        return [];
      }
    },

    // 6. Mengambil progress kelengkapan berkas
    getDocumentProgress: async function (applicationId = '33333333-3333-3333-3333-333333333333') {
      if (!supabaseClient) return null;
      try {
        const { data, error } = await supabaseClient
          .from('v_candidate_document_progress')
          .select('*')
          .eq('application_id', applicationId)
          .single();

        if (error) throw error;
        return data;
      } catch (err) {
        console.error('[PPDS Supabase] Gagal mengambil progress dokumen:', err.message);
        return null;
      }
    },

    // 7. Mengambil jadwal wawancara resmi (Tahap 3)
    getInterviewSchedule: async function (applicationId = '33333333-3333-3333-3333-333333333333') {
      if (!supabaseClient) return null;
      try {
        const { data, error } = await supabaseClient
          .from('interview_schedules')
          .select('*')
          .eq('application_id', applicationId)
          .single();

        if (error) throw error;
        return data;
      } catch (err) {
        console.error('[PPDS Supabase] Gagal mengambil jadwal wawancara:', err.message);
        return null;
      }
    },

    // 8. Mengambil Surat Keputusan (SK Yudisium) resmi
    getDecree: async function (applicationId = '33333333-3333-3333-3333-333333333333') {
      if (!supabaseClient) return null;
      try {
        const { data, error } = await supabaseClient
          .from('decrees')
          .select('*')
          .eq('application_id', applicationId)
          .eq('is_published', true)
          .single();

        if (error) throw error;
        return data;
      } catch (err) {
        console.error('[PPDS Supabase] Gagal mengambil SK Yudisium:', err.message);
        return null;
      }
    },

    // 9. Mengambil warta & pengumuman
    getAnnouncements: async function () {
      if (!supabaseClient) return [];
      try {
        const { data, error } = await supabaseClient
          .from('portal_announcements')
          .select('*')
          .order('publish_date', { ascending: false });

        if (error) throw error;
        return data || [];
      } catch (err) {
        console.error('[PPDS Supabase] Gagal mengambil pengumuman:', err.message);
        return [];
      }
    },

    // 10. Fungsi Unggah Berkas ke Supabase Storage (candidate-documents)
    uploadDocument: async function (applicationId, requirementId, file) {
      if (!supabaseClient) throw new Error("Supabase belum terhubung.");

      const fileExt = file.name.split('.').pop();
      const fileName = `${applicationId}/${Date.now()}_${requirementId}.${fileExt}`;

      // Upload ke Storage
      const { data: storageData, error: storageError } = await supabaseClient.storage
        .from('candidate-documents')
        .upload(fileName, file, { upsert: true });

      if (storageError) throw storageError;

      const { data: urlData } = supabaseClient.storage
        .from('candidate-documents')
        .getPublicUrl(fileName);

      // Simpan referensi ke applicant_documents
      const { data: docData, error: docError } = await supabaseClient
        .from('applicant_documents')
        .upsert({
          application_id: applicationId,
          requirement_id: requirementId,
          file_name: file.name,
          storage_path: fileName,
          file_url: urlData.publicUrl,
          file_size_kb: Math.round(file.size / 1024),
          file_format: fileExt.toUpperCase(),
          validation_status: 'pending',
          uploaded_at: new Date().toISOString()
        }, { onConflict: 'application_id,requirement_id' });

      if (docError) throw docError;
      return docData;
    }
  };
})();
