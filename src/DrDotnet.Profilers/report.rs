use std::path::{PathBuf, Path};
use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

use crate::rust_protobuf_protos::interop::*;

pub trait Session {
    fn create_session_json(&self);
    fn create_report(&self, filename: String) -> Report;
    fn get_root_directory() -> String;
    fn get_directory(session_id: &String) -> String;
}

impl Session for SessionInfo {

    // Returns a Session from its UID and ProfilerData.
    // If the Session report is not present on the disk, it will be written at the same time.
    fn create_session_json(&self) {

        let s = SessionInfo::new();

        // Serialize to JSON
        let json = protobuf_json_mapping::print_to_string(&s).unwrap();

        // Write session report
        let json_path = format!("{}/session.json", SessionInfo::get_directory(&self.uuid));
        if !Path::exists(Path::new(&json_path)) {
            let mut session_stream = File::create(json_path).expect("Unable to create file");
            session_stream.write_all(json.as_bytes()).expect("Unable to write data");    
        };
    }

    // Create a new report for a given Session, ready to be filled up.
    fn create_report(&self, filename: String) -> Report {
        self.create_session_json(); // Could be done once instead of once per report written
        let path = PathBuf::from(format!(r"{}/{}", SessionInfo::get_directory(&self.uuid), filename));
        let file = File::create(&path).unwrap();
        return Report { writer: BufWriter::new(file) };
    }

    fn get_root_directory() -> String {
        let directory_path = format!(r"{}/dr-dotnet", std::env::temp_dir().into_os_string().into_string().unwrap());
        std::fs::create_dir_all(&directory_path);
        return directory_path;
    }
    
    // Returns the directy path for this Session.
    fn get_directory(session_id: &String) -> String {
        let directory_path = format!(r"{}/{}", SessionInfo::get_root_directory(), session_id.to_string());
        std::fs::create_dir_all(&directory_path);
        return directory_path;
    }
}

// A Session can contain several reports, which are usually files like markdown summaries or charts.
pub struct Report {
    pub writer: BufWriter<File>,
}

impl Report {
    pub fn write_line(&mut self, text: String) {
        self.writer.write(format!("{}\r\n", text).as_bytes()).unwrap();
    }

    pub fn new_line(&mut self) {
        self.writer.write(b"\r\n").unwrap();
    }
}