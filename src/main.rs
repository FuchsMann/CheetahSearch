mod search;
mod search_hit;

use search::Search;

fn main() {
    println!("S INIT");
    let mut search = Search::new(
        String::from("C:\\Users\\timst"),
        vec![String::from("sussy baka")],
        vec![String::from(".txt"), String::from("json")],
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