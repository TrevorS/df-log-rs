use eframe::egui;

use egui::text::{LayoutJob, TextFormat};
use egui::Color32;

// TODO: Implement more complex layout caching.
pub struct CachingHighlighter {
    is_dark_mode: bool,
    string: String,
    output: LayoutJob,
    highlighter: Highlighter,
}

impl Default for CachingHighlighter {
    fn default() -> Self {
        Self {
            is_dark_mode: false,
            string: "".to_owned(),
            output: LayoutJob::default(),
            highlighter: Highlighter {},
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

pub struct Highlighter {}

impl Highlighter {
    pub fn highlight(&mut self, _is_dark_mode: bool, string: &str) -> LayoutJob {
        let format = create_text_format(Color32::WHITE, Color32::BLACK);

        let mut job = LayoutJob::default();
        job.append(&string, 0.0, format);

        let fa = create_text_format(Color32::RED, Color32::BLACK);
        job.append("\nHello Faith-Anne!", 0.0, fa);

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
