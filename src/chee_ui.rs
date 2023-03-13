use eframe::egui;
use num_cpus::get;
use egui::{Button, Widget};
use egui_extras::{Column, TableBuilder};

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

struct MyApp {
    pub keyword: String,
    pub file_extensions: Vec<String>,
    pub search_hits: u32,
    pub cores_max: u32,
    pub cores: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            keyword: "keyword".to_string(),
            file_extensions: vec!["txt".to_string()],
            search_hits: 0,
            cores_max: get() as u32,
            cores: 0
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
                let btn_search = egui::Button::new("Search").ui(ui);
                if btn_search.clicked() {
                    println!("Clicked");
                };

                egui::Slider::new(&mut self.cores, 0..=self.cores_max)
                    .ui(ui);
            });

            let table = TableBuilder::new(ui)
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
                });
            table.body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("test.js");
                    });
                    row.col(|ui| {
                        ui.label(String::new());
                    });
                });
                for i in 0..100 as u32 {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(i.to_string());
                        });
                        row.col(|ui| {
                            ui.label("cheese");
                        });
                    });
                }
            });
        });
    }
}
