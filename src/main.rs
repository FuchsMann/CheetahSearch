#![windows_subsystem = "windows"]

mod search;
mod search_hit;
mod chee_ui;

//use search::Search;

use chee_ui::app_init;

fn main() {
    app_init().unwrap();
}

/* 
fn main() {
    println!("S INIT");
    let mut search = Search::new(
        String::from("/Users/timstammwitz/Documents"),
        vec![String::from("search")],
        vec![String::from(".rs"), String::from("cheese")],
    );

    println!("Starting Index.");
    search.index_filepaths();
    println!("Starting search.");
    search.parallel_search();
    println!("Hits: {}", search.search_hits.len());
    for search_hit in &search.search_hits {
        println!("{}", search_hit.to_string());
    }
}
*/