use std::sync::mpsc::TryRecvError;

use eframe::{egui, epi};
use egui::{FontDefinitions, FontFamily};

use crate::event::{Event, EventReceiver};
use crate::highlighter::CachingHighlighter;
use crate::settings::Settings;

pub struct App {
    lines: Vec<String>,
    highlighter: CachingHighlighter,
    rx: EventReceiver,
}

impl App {
    pub fn new(settings: Settings, rx: EventReceiver) -> Self {
        Self {
            lines: vec![],
            highlighter: CachingHighlighter::new(settings),
            rx,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "DF-Log-RS"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "cascadia".into(),
            std::borrow::Cow::Borrowed(include_bytes!(
                "../fonts/CascadiaCode-2108.26/ttf/CascadiaCode.ttf"
            )),
        );

        fonts
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "cascadia".into());

        ctx.set_fonts(fonts);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            lines,
            highlighter,
            rx,
        } = self;

        match rx.try_recv() {
            Ok(event) => match event {
                Event::Announcement(text) => lines.push(text),
                Event::InitialLog(mut new_lines) => {
                    lines.clear();
                    lines.append(&mut new_lines);
                }
            },
            Err(TryRecvError::Empty) => {}
            Err(e) => {
                panic!("{}", e);
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Quit").clicked() {
                    frame.quit();
                }

                if ui.button("Clear").clicked() {
                    lines.clear();
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let text_style = eframe::egui::TextStyle::Body;
            let row_height = ui.fonts()[text_style].row_height() + ui.spacing().item_spacing.y;
            let num_rows = lines.len();

            let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                let mut layout_job = highlighter.highlight(ui.visuals().dark_mode, string);
                layout_job.wrap_width = wrap_width;

                ui.fonts().layout_job(layout_job)
            };

            egui::ScrollArea::vertical().show_rows(ui, row_height, num_rows, |ui, row_range| {
                let mut text: String;

                // INFO: Ran into panic where index was larger than length after clearing buffer.
                if lines.is_empty() {
                    text = "".into();
                } else {
                    text = lines[row_range].join("\n");
                }

                let log = egui::TextEdit::multiline(&mut text)
                    .desired_width(f32::INFINITY)
                    .enabled(false)
                    .frame(false)
                    .layouter(&mut layouter);

                ui.add_sized(ui.available_size(), log);
            });
        });
    }
}
