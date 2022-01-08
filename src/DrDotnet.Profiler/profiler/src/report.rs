use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local};

use super::profilers::ProfilerData;

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub guid: Uuid,
    pub name: String,
    pub timestamp: DateTime<Local>,
    pub profiler: ProfilerData,
    pub sections: Vec<ReportSection>,
}

#[derive(Serialize, Deserialize)]
pub struct ReportSection {
    pub name: String,
    pub entries: Vec<ReportEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct ReportEntry {
    pub name: String,
    pub content: String,
}