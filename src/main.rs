#![windows_subsystem = "windows"]

mod search;
mod search_hit;
mod chee_ui;

//use search::Search;

use chee_ui::app_init;

fn main() {
    app_init().unwrap();
}