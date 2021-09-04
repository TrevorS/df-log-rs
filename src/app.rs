use eframe::{egui, epi};

use crate::highlighter::CachingHighlighter;
use crate::settings::Settings;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default()))]
pub struct App {
    lines: Vec<String>,
    highlighter: CachingHighlighter,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        Self {
            lines: vec![
                "Trevor has been stung by a bee!".into(),
                "Ada has been stung by a bee!".into(),
                "Faith-Anne has been stung by a bee!".into(),
            ],
            highlighter: CachingHighlighter::new(settings),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "DF-Log-RS"
    }

    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Quit").clicked() {
                    frame.quit();
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let Self { lines, highlighter } = self;

            let text_style = eframe::egui::TextStyle::Body;
            let row_height = ui.fonts()[text_style].row_height();
            let num_rows = lines.len();

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = highlighter.highlight(ui.visuals().dark_mode, string);
                layout_job.wrap_width = wrap_width;

                ui.fonts().layout_job(layout_job)
            };

            egui::ScrollArea::vertical().show_rows(ui, row_height, num_rows, |ui, row_range| {
                let mut text = lines[row_range].join("\n");
                let log = egui::TextEdit::multiline(&mut text)
                    .text_style(text_style)
                    .desired_width(f32::INFINITY)
                    .enabled(false)
                    .frame(false)
                    .layouter(&mut layouter);

                ui.add(log);
            });
        });
    }
}
