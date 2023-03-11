pub struct Search {
    pub root_folder: String,
    pub keywords: Vec<String>,
    pub file_extensions: Vec<String>,
}

// function to create a new search

impl Search {
    pub fn new(root_folder: String, keywords: Vec<String>, file_extensions: Vec<String>) -> Search {
        Search {
            root_folder,
            keywords,
            file_extensions
        }
    }
}