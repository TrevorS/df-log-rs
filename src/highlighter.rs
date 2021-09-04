use eframe::egui::text::{LayoutJob, TextFormat};
use eframe::egui::Color32;

use crate::settings::Settings;

// TODO: Implement more sophisticated layout caching.
pub struct CachingHighlighter {
    is_dark_mode: bool,
    string: String,
    output: LayoutJob,
    highlighter: Highlighter,
}

impl CachingHighlighter {
    pub fn new(settings: Settings) -> Self {
        Self {
            is_dark_mode: false,
            string: "".into(),
            output: LayoutJob::default(),
            highlighter: Highlighter::new(settings),
        }
    }
}

impl CachingHighlighter {
    pub fn highlight(&mut self, is_dark_mode: bool, string: &str) -> LayoutJob {
        if (is_dark_mode, string) != (self.is_dark_mode, &self.string) {
            self.output = self.highlighter.highlight(is_dark_mode, string);
        }

        self.output.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ParsedLine {
    line: String,
    group: Option<String>,
    category: Option<String>,
    color: Option<String>,
    highlights: Vec<(String, String)>,
    icons: Vec<(String, String)>,
}

impl ParsedLine {
    pub fn get_words(&self) -> Vec<&str> {
        self.line.split_whitespace().collect()
    }

    pub fn get_base_text_color(&self) -> Color32 {
        if let Some(hex) = &self.color {
            hex_to_color(&hex)
        } else {
            Color32::WHITE
        }
    }
}

pub struct Highlighter {
    settings: Settings,
}

impl Highlighter {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    pub fn parse_line(&self, line: &str) -> ParsedLine {
        let line = String::from(line);

        let mut highlights = vec![];
        let mut icons = vec![];

        for (word, color) in self.settings.get_highlights() {
            if line.contains(word) {
                let color = self.settings.translate_color(&color);

                highlights.push((word.to_owned(), color));
            }
        }

        for (word, icon) in self.settings.get_icons() {
            if line.contains(word) {
                icons.push((word.to_owned(), icon.to_owned()))
            }
        }

        for filter in self.settings.get_filters() {
            if filter.matches(&line) {
                return ParsedLine {
                    line,
                    group: Some(filter.group.to_owned()),
                    category: Some(filter.category.to_owned()),
                    color: filter.color.to_owned(),
                    highlights,
                    icons,
                };
            }
        }

        ParsedLine {
            line,
            group: None,
            category: None,
            color: None,
            highlights,
            icons,
        }
    }

    pub fn highlight(&mut self, _is_dark_mode: bool, line: &str) -> LayoutJob {
        let mut job = LayoutJob::default();
        let parsed_line = self.parse_line(line);

        dbg!(&parsed_line);

        let text_format = create_text_format(parsed_line.get_base_text_color(), Color32::BLACK);
        job.append(&parsed_line.line, 0.0, text_format);

        job
    }
}

pub fn create_text_format(foreground: Color32, background: Color32) -> TextFormat {
    let mut format = TextFormat::default();

    format.color = foreground;
    format.background = background;

    format
}

pub fn hex_to_color(hex: &str) -> Color32 {
    let r = usize::from_str_radix(&hex[1..3], 16).unwrap() as u8;
    let g = usize::from_str_radix(&hex[3..5], 16).unwrap() as u8;
    let b = usize::from_str_radix(&hex[5..7], 16).unwrap() as u8;

    Color32::from_rgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_color_white() {
        let hex = "#ffffff";
        let color = hex_to_color(hex);

        assert_eq!(color.r(), 255);
        assert_eq!(color.g(), 255);
        assert_eq!(color.b(), 255);
    }

    #[test]
    fn test_hex_to_color_black() {
        let hex = "#000000";
        let color = hex_to_color(hex);

        assert_eq!(color.r(), 0);
        assert_eq!(color.g(), 0);
        assert_eq!(color.b(), 0);
    }

    #[test]
    fn test_hex_to_color_cool_green() {
        let hex = "#ddeecc";
        let color = hex_to_color(hex);

        assert_eq!(color.r(), 221);
        assert_eq!(color.g(), 238);
        assert_eq!(color.b(), 204);
    }
}
