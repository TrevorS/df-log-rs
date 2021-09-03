use eframe::egui;

use egui::text::{LayoutJob, TextFormat};
use egui::Color32;

pub struct Highlighter {
    is_dark_mode: bool,
    string: String,
    output: LayoutJob,
}

impl Highlighter {
    pub fn highlight(&mut self, is_dark_mode: bool, string: &str) -> LayoutJob {
        if (is_dark_mode, string) != (self.is_dark_mode, &self.string) {
            let monospace = egui::TextStyle::Monospace;

            let mut normal_format = TextFormat::default();
            normal_format.style = monospace;
            normal_format.color = Color32::WHITE;
            normal_format.background = Color32::BLACK;

            let mut job = LayoutJob::default();
            job.append(&string, 0.0, normal_format);

            let mut fa = TextFormat::default();
            fa.style = monospace;
            fa.color = Color32::RED;
            fa.background = Color32::BLACK;

            job.append("\nHello Faith-Anne!", 0.0, fa);

            self.is_dark_mode = is_dark_mode;
            self.output = job;
        }

        self.output.clone()
    }
}

impl Default for Highlighter {
    fn default() -> Self {
        Self {
            is_dark_mode: false,
            string: "".to_owned(),
            output: LayoutJob::default(),
        }
    }
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
