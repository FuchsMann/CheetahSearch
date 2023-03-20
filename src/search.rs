use crate::search_hit::SearchHit;
use std::{thread::{self, JoinHandle}, io::{BufRead, BufReader}, fs::File};

pub struct Search {
    pub root_folder: String,
    pub keywords: Vec<String>,
    pub file_extensions: Vec<String>,

    pub qualified_filepaths: Vec<String>,
    pub search_hits: Vec<SearchHit>,
}

impl Search {
    pub fn new(root_folder: String, keywords: Vec<String>, file_extensions: Vec<String>) -> Search {
        Search {
            root_folder,
            keywords,
            file_extensions,
            qualified_filepaths: Vec::new(),
            search_hits: Vec::new(),
        }
    }

    pub fn index_filepaths(&mut self) {
        let mut filepaths = Vec::new();
        let mut filepaths_to_check = Vec::new();
        filepaths_to_check.push(self.root_folder.clone());

        while filepaths_to_check.len() > 0 {
            let current_filepath = filepaths_to_check.pop().unwrap();
            let mut current_dir = match std::fs::read_dir(current_filepath) {
                Ok(dir) => dir,
                Err(_) => continue,
            };
            while let Some(entry) = current_dir.next() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    filepaths_to_check.push(path.to_str().unwrap().to_string());
                } else {
                    filepaths.push(path.to_str().unwrap().to_string());
                }
            }
        }

        for filepath in filepaths {
            for file_extension in &self.file_extensions {
                if filepath.ends_with(file_extension) {
                    self.qualified_filepaths.push(filepath.clone());
                    continue;
                }
            }
        }
    }

    pub fn single_file_search(filepath: String, keywords: Vec<String>) -> Vec<SearchHit> {

        fn get_col(line: String, keyword: String) -> u32 {
            let mut col = 0;
            for c in line.chars() {
                if c == keyword.chars().next().unwrap() {
                    break;
                }
                col += 1;
            }
            col
        }

        let mut search_hits = Vec::new();
        let file = match File::open(filepath.clone()) {
            Ok(file) => file,
            Err(_) => return search_hits,
        };
        let fr = BufReader::new(file).lines(); 

        for (line_nr, line) in fr.enumerate() {
            let line = match line {
                Ok(line) => line,
                Err(_) => continue,
            };
            for keyword in &keywords {
                if line.contains(keyword) {
                    let search_hit = SearchHit::new(filepath.clone(), line.clone(), line_nr as u32, get_col(line.clone(), keyword.clone()));
                    search_hits.push(search_hit);
                }
            }
        }
        search_hits
    }

    pub fn parallel_search(&mut self, cores: u32) {
        let mut handles: Vec<JoinHandle<Vec<SearchHit>>> = Vec::new();
        let nr_of_threads: u32 = cores;
        let mut thread_ctr: u32 = 0;

        for filepath in &self.qualified_filepaths {
            let fp = filepath.clone();
            let kw = self.keywords.clone();
            thread_ctr += 1;
            let handle = thread::spawn(move || {
                Search::single_file_search(fp, kw)
            });
            handles.push(handle);
            if thread_ctr >= nr_of_threads {
                for handle in handles {
                    let search_hits = handle.join().unwrap();
                    for search_hit in search_hits {
                        self.search_hits.push(search_hit);
                    }
                }
                handles = Vec::new();
                thread_ctr = 0;
            }
        }
        for handle in handles {
            let search_hits = handle.join().unwrap();
            for search_hit in search_hits {
                self.search_hits.push(search_hit);
            }
        }
    }

}