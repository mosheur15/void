mod tokens;
use std::fs;


/// A structure representing a file buffer with tracking and tokemization capabilities.
#[derive(Debug)]
pub struct File {
    pub filename: String,
    pub raw: Vec<u8>,
    pub cursor: usize,
    pub line_mapping: Vec<usize>,
}

impl File {
    /// Creates a new file buffer.
    ///
    /// # Arguments
    /// * `filename` - A `String` containing the name/path of the file.
    pub fn new(filename: String) -> Self {
        let mut file = Self {
            filename: filename,
            raw: vec![],
            cursor: 0,
            line_mapping: vec![],
        };
        file.read();
        return file;
    }

    /// Reads a file and load the bytes to `File.raw`
    pub fn read(&mut self) {
        let mut empty_err_msg = "".to_string();
        let read_result = fs::read(&self.filename).unwrap();
        self.raw = read_result;
    }

    /// Figures out the line number by a byte position.
    /// 
    /// # Returns
    /// * `.0` - A `usize` as the line number.
    /// * `.1` - A `usize` as the reletive position starting from the line number.
    pub fn get_line_number(&self, byte_position: &usize) -> (usize, usize) {
        return (
            10 as usize,
            20 as usize,
        );
    }
}

#[cfg(test)]
mod mod_tests; 
