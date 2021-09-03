use eframe::{egui, epi};

use crate::highlighter::CachingHighlighter;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default()))]
pub struct App {
    line: String,
    highlighter: CachingHighlighter,
}

impl Default for App {
    fn default() -> Self {
        Self {
            line: "Ada has become a Cook.".into(),
            highlighter: CachingHighlighter::default(),
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
            let Self { line, highlighter } = self;

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = highlighter.highlight(ui.visuals().dark_mode, string);

                layout_job.wrap_width = wrap_width;
                ui.fonts().layout_job(layout_job)
            };

            let announcements = egui::TextEdit::multiline(line)
                .text_style(egui::TextStyle::Monospace)
                .enabled(false)
                .frame(false)
                .desired_width(f32::INFINITY)
                .layouter(&mut layouter);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(ui.available_size(), announcements);
            });
        });
    }
}
