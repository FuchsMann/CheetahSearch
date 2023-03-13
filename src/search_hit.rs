pub struct SearchHit {
    pub file_path: String,
    pub line_content: String,
    pub line_nr: u32,
    pub col_nr: u32
}

impl SearchHit {
    pub fn new(file_path: String, line_content: String, line_nr: u32, col_nr: u32) -> SearchHit {
        SearchHit {
            file_path,
            line_content,
            line_nr,
            col_nr
        }
    }
}

