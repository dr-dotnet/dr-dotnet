use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Local};

use std::path::PathBuf;
use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

use crate::profilers::ProfilerData;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    session_id: Uuid,
    process_name: String,
    timestamp: DateTime<Local>,
    profiler: ProfilerData,
}

impl Session {

    pub fn create_session(session_id: Uuid, profiler: ProfilerData) -> Session {

        let process_name = std::env::current_exe().unwrap()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_owned();

        let report = Session {
            session_id: session_id,
            process_name : process_name,
            profiler: profiler,
            timestamp: chrono::offset::Local::now()
        };

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&report).unwrap();

        // Write session report
        std::fs::create_dir_all(report.get_directory());
        let mut session_stream = File::create(format!("{}/session.json", report.get_directory())).expect("Unable to create file");
        session_stream.write_all(json.as_bytes()).expect("Unable to write data");    

        return report;
    }

    pub fn create_report(&self, filename: String) -> Report {
        let path = PathBuf::from(format!(r"{}/{}", self.get_directory(), filename));
        let file = File::create(&path).unwrap();
        return Report { writer: BufWriter::new(file) };
    }

    pub fn get_directory(&self) -> String {
        return format!(r"/dr-dotnet/{}", self.session_id.to_string());
    }
}

pub struct Report {
    pub writer: BufWriter<File>,
}

impl Report {
    pub fn write_line(&mut self, text: String) {
        self.writer.write(format!("{}\n", text).as_bytes()).unwrap();
    }
}