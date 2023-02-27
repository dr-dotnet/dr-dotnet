use std::io::BufWriter;
use std::fs::File;
use std::io::Write;

// A Session can contain several reports, which are usually files like markdown summaries or charts.
pub struct Report {
    pub writer: BufWriter<File>,
}

impl Report {
    pub fn write_line(&mut self, text: String) {
        self.writer.write(format!("{}\r\n", text).as_bytes()).unwrap();
    }

    pub fn write<S>(&mut self, text: S) where S: Into<String> {
        self.writer.write(text.into().as_bytes()).unwrap();
    }

    pub fn new_line(&mut self) {
        self.writer.write(b"\r\n").unwrap();
    }
}