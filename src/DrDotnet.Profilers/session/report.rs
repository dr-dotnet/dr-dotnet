use std::io::{BufRead, BufReader, BufWriter, Result, Seek, SeekFrom, Write};
use std::fs::File;
use std::path::PathBuf;

// A Session can contain several reports, which are usually files like markdown summaries or charts.
pub struct Report {
    pub writer: BufWriter<File>,
    pub(crate) filepath: PathBuf
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

    pub fn reverse_lines(&mut self) -> Result<()> {
        self.writer.flush()?;

        let mut reader = BufReader::new(File::open(&self.filepath)?);
        let mut lines = Vec::new();

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            lines.push(line);
        }

        self.writer.get_mut().seek(SeekFrom::Start(0))?;

        for line in lines.iter().rev() {
            self.writer.write_all(line.as_bytes())?;
        }

        self.writer.flush()?;

        Ok(())
    }

}