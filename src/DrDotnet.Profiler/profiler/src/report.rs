use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local};

use profiling_api::{ClrProfiler};

#[derive(serde::Serialize, Deserialize)]
pub struct Report {
    pub guid: Uuid,
    pub name: String,
    pub timestamp: DateTime<Local>,
    pub profiler: ProfilerData,
    pub sections: Vec<ReportSection>,
}

#[derive(serde::Serialize, Deserialize)]
pub struct ProfilerData {
    pub guid: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(serde::Serialize, Deserialize)]
pub struct ReportSection {
    pub name: String,
    pub entries: Vec<ReportEntry>,
}

#[derive(serde::Serialize, Deserialize)]
pub struct ReportEntry {
    pub name: String,
    pub content: String,
}

impl ProfilerData {
    pub fn from_profiler<T: ClrProfiler>() -> Self {
        ProfilerData {
            guid: T::get_guid(),
            name: T::get_name(),
            description: T::get_description()
        }
    }
}