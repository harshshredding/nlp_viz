use eframe::{run_native, epi::App, egui, NativeOptions};
use serde_json::Value;
use std::fs;

struct FileRawText {
    samples: Vec<String>,
}

impl FileRawText {
    pub fn new(samples: Vec<String>) -> Self {
        FileRawText{samples: samples}
    }
}

impl App for FileRawText {

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                ui.label("side panel");
            });
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading("NLP Visualizer");
                for sample in &(self.samples) {
                    ui.separator();
                    ui.label(sample);
                }
            }); 
    }

    fn name(&self) -> &str {
        "NLPGui"
    }
}

fn read_training_data() -> Value {
    let training_file_path = "/Users/harshverma/Downloads/preprocessed_data/legaleval_train_judgement_sample_text.json";
    let file_content = fs::read_to_string(training_file_path).expect("Should have been able to read the file.");
    serde_json::from_str(&file_content).expect("not able to parse json")
}

fn main() {
    let all_samples_map = read_training_data();
    let all_samples_map = match all_samples_map {
        Value::Object(map) => map,
        _ => panic!("should have been a map")
    };
    let samples_list: Vec<String> = all_samples_map.values().map(|x| x.to_string()).collect();
    let app = Box::new(FileRawText::new(samples_list));
    let native_options = NativeOptions::default();
    run_native(app, native_options);
}