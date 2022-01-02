mod profilers;
mod report;

use serde_json::Result;
use std::fs::File;
use std::io::Write;
use report::*;
use profilers::ExceptionsProfiler;

#[no_mangle]
pub extern "C" fn string_from_rust() -> *const i8 {

    let mut report = report::Report{
        guid: uuid::Uuid::default(),
        name: String::default(),
        timestamp: chrono::offset::Local::now(),
        profiler: ProfilerData::from_profiler::<ExceptionsProfiler>(),
        sections: vec![]
    };

    report.name = "myName".to_owned();

    let json = serde_json::to_string_pretty(&report).unwrap();
    println!("current dir: {}", std::env::current_dir().unwrap().display());
    std::fs::create_dir_all("./tmp");

    let mut f = File::create("./tmp/report.json").expect("Unable to create file");
    f.write_all(json.as_bytes()).expect("Unable to write data");

    let s = std::ffi::CString::new("Hello World").unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);

    println!("done!");

    return p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_from_rust() {
        string_from_rust();
        assert!(true);
    }
}