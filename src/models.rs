use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CandidateProfile {
    pub registration_number: String,
    pub full_name: String,
    pub program_name: String,
    pub hospital_affiliation: String,
    pub quota: u32,
    pub current_stage: String,
    pub overall_status: String,
    pub nik: String,
    pub str_number: String,
    pub str_valid_until: String,
    pub idi_number: String,
    pub email: String,
    pub phone: String,
}

impl Default for CandidateProfile {
    fn default() -> Self {
        Self {
            registration_number: "REG-PPDS-2025-0892".to_string(),
            full_name: "dr. Agung Setiawan, S.Ked".to_string(),
            program_name: "Spesialis Anestesiologi dan Terapi Intensif (Sp.An-TI)".to_string(),
            hospital_affiliation: "Fakultas Kedokteran Universitas Tanjungpura / RSUD Dokter Soedarso Pontianak".to_string(),
            quota: 2,
            current_stage: "Tahap 3 • Wawancara Terjadwal".to_string(),
            overall_status: "LULUS TAHAP 2 (AKADEMIK & KESEHATAN)".to_string(),
            nik: "6171012408940003".to_string(),
            str_number: "31.1.1.100.1.20.219842".to_string(),
            str_valid_until: "14 November 2028".to_string(),
            idi_number: "NPA-IDI: 148291-KB".to_string(),
            email: "agung.setiawan.md@alumni.untan.ac.id".to_string(),
            phone: "+62 813-4521-8900".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StageItem {
    pub stage_number: u32,
    pub title: String,
    pub subtitle: String,
    pub date: String,
    pub description: String,
    pub status: StageStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum StageStatus {
    Completed,
    Current,
    Upcoming,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EvaluationScore {
    pub component_name: String,
    pub category: String,
    pub score_numeric: Option<f64>,
    pub score_qualitative: Option<String>,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentItem {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub format: String,
    pub size: String,
    pub upload_date: String,
    pub status: String,
    pub status_color: String,
    pub note: String,
    pub icon: String,
    pub description: String,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterviewSchedule {
    pub date_formatted: String,
    pub time_formatted: String,
    pub location: String,
    pub room: String,
    pub panel_title: String,
    pub panel_members: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnouncementItem {
    pub id: String,
    pub title: String,
    pub content: String,
    pub date: String,
    pub is_highlight: bool,
}
