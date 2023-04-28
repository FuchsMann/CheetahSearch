use eframe::egui;
use egui::Widget;
use egui_extras::{Column, TableBuilder};
use num_cpus::get;
use rfd::FileDialog;
use std::path::PathBuf;

use crate::search::Search;
use crate::search_hit::SearchHit;

pub fn app_init() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Cheetah Search",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

pub fn ask_folder() -> String {
    let dialog = FileDialog::new();
    let folder: PathBuf = match dialog.pick_folder().as_deref() {
        Some(folder) => folder.into(),
        None => return "".to_string(),
    };
    match folder.to_str() {
        Some(folder) => folder.to_string(),
        None => "".to_string(),
    }
}

pub fn run_search(
    keyword: String,
    search_path: String,
    file_extensions: String,
    cores: u32,
) -> Vec<SearchHit> {
    let mut search = Search::new(
        search_path,
        vec![keyword],
        file_extensions
            .split_whitespace()
            .map(|s| s.to_string())
            .collect(),
    );

    search.index_filepaths();
    search.parallel_search(cores);
    search.search_hits
}

struct MyApp {
    pub keyword: String,
    pub search_path: String,
    pub file_extensions: String,
    pub search_hits: Vec<SearchHit>,
    pub search_hits_ctr: u32,
    pub cores_max: u32,
    pub cores: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            keyword: "".to_string(),
            search_path: "".to_string(),
            file_extensions: "".to_string(),
            search_hits: Vec::new(),
            search_hits_ctr: 0,
            cores_max: get() as u32,
            cores: get() as u32,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Search ready");

            ui.horizontal(|ui| {
                egui::Label::new("Keyword: ").ui(ui);
                egui::TextEdit::singleline(&mut self.keyword)
                    .horizontal_align(egui::Align::Max)
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });

            ui.horizontal(|ui| {
                if egui::Button::new("Path").ui(ui).clicked() {
                    self.search_path = ask_folder();
                }
                egui::TextEdit::singleline(&mut self.search_path)
                    .horizontal_align(egui::Align::Max)
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });

            ui.horizontal(|ui| {
                let btn_search = egui::Button::new("Search").ui(ui);
                if btn_search.clicked() {
                    self.search_hits = run_search(
                        self.keyword.clone(),
                        self.search_path.clone(),
                        self.file_extensions.clone(),
                        self.cores,
                    );
                    self.search_hits_ctr = self.search_hits.len() as u32;
                };
                egui::Slider::new(&mut self.cores, 1..=self.cores_max).ui(ui);
                egui::Label::new("Extensions: ").ui(ui);
                egui::TextEdit::singleline(&mut self.file_extensions)
                    .horizontal_align(egui::Align::Max)
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });

            TableBuilder::new(ui)
                .striped(true)
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Hits");
                    });
                    header.col(|ui| {
                        ui.heading("Filename");
                    });
                })
                .body(|mut body| {
                    // body.row(30.0, |mut row| {
                    //     row.col(|ui| {
                    //         ui.label(String::new());
                    //     });
                    //     row.col(|ui| {
                    //         ui.label(String::new());
                    //     });
                    // });
                    for (i, hit) in self.search_hits.iter().enumerate() {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(i.to_string());
                            });
                            row.col(|ui| {
                                ui.label(hit.file_path.clone());
                            });
                        });
                    }
                });
        });
    }
}
