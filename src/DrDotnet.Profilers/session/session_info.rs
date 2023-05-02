use std::path::{PathBuf, Path};
use std::io::BufWriter;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use crate::rust_protobuf_protos::interop::*;

use crate::session::Report;

impl SessionInfo {

    // Returns a Session from its UID and ProfilerData.
    // If the Session report is not present on the disk, it will be written at the same time.
    pub fn create_session_json(&self) {

        // Serialize to JSON
        let json = protobuf_json_mapping::print_to_string(self).unwrap();

        // Write session report
        let json_path = format!("{}/session.json", SessionInfo::get_directory(&self.uuid));
        if !Path::exists(Path::new(&json_path)) {
            let mut session_stream = File::create(json_path).expect("Unable to create file");
            session_stream.write_all(json.as_bytes()).expect("Unable to write data");    
        };
    }

    // Create a new report for a given Session, ready to be filled up.
    pub fn create_report(&self, filename: String) -> Report {
        self.create_session_json(); // Could be done once instead of once per report written
        let path = PathBuf::from(format!(r"{}/{}", SessionInfo::get_directory(&self.uuid), filename));
        let file = File::create(&path).unwrap();
        return Report { writer: BufWriter::new(file), filepath: path  };
    }

    pub fn get_root_directory() -> String {
        let directory_path = format!(r"{}/dr-dotnet", std::env::temp_dir().into_os_string().into_string().unwrap());
        std::fs::create_dir_all(&directory_path).ok();
        return directory_path;
    }
    
    // Returns the directy path for this Session.
    pub fn get_directory(session_id: &String) -> String {
        let directory_path = format!(r"{}/{}", SessionInfo::get_root_directory(), session_id.to_string());
        std::fs::create_dir_all(&directory_path).ok();
        return directory_path;
    }

    pub fn init(data: *const std::os::raw::c_void, data_length: u32) -> Result<Self, &'static str> {

        if data_length <= 0 {
            return Err("Data should be non empty to carry the session ID");
        }
    
        let buffer: &[u8] = unsafe { std::slice::from_raw_parts(data as *const u8, data_length as usize) };
        let session_info_result: Result<SessionInfo, protobuf::Error> = protobuf::Message::parse_from_bytes(&buffer);
    
        match session_info_result {
            Ok(session_info) => {
                info!("Successfully parsed session with ID {}", session_info.uuid);
                Ok(session_info)
            },
            Err(_) => {
                Err("Failed to parse session ID from FFI data")
            }
        }
    }

    pub fn get_parameter<T: FromStr>(&self, key: &str) -> Result<T, String>{
        match self.profiler.parameters.iter().find(|&x| x.key == key) {
            Some(property) => match property.value.to_lowercase().parse::<T>() {
                Ok(value) => Ok(value),
                Err(_) => Err(format!("Could not convert property '{}' value '{}' to expected type", key, property.value)),
            },
            None => Err(format!("Could not find property '{}'", key)),
        }
    }
}