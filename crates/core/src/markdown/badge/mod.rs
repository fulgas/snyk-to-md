use crate::model::security_report::Severity;
use svg::node::element::{Definitions, LinearGradient, Path, Rectangle, Stop, Text};
use svg::Document;

pub struct BadgeFactory;

impl BadgeFactory {
    fn generate_svg_badge(severity: &Severity, count: usize) -> String {
        let (color, status_text) = match severity {
            Severity::Critical => ("#e05d44", "Critical"),
            Severity::High => ("#fe7d37", "High"),
            Severity::Medium => ("#dfb317", "Medium"),
            Severity::Low => ("#4c1", "Low"),
        };

        let full_text = format!("{}: {}", status_text, count);
        let label_text = "security";
        let label_width = 60;
        let value_width = 10 + (full_text.len() as i32 * 7); // Rough estimate
        let total_width = label_width + value_width;
        let text_x = label_width + (value_width / 2);

        // --- SVG Building ---
        let gradient_id = "a";

        let document = Document::new()
            .set("width", total_width)
            .set("height", 20)
            .set("xmlns", "http://www.w3.org/2000/svg")
            .add(
                // Gradient definition
                Definitions::new().add(
                    LinearGradient::new()
                        .set("id", gradient_id)
                        .set("x2", "0")
                        .set("y2", "100%")
                        .add(
                            Stop::new()
                                .set("offset", "0%")
                                .set("stop-color", "#bbb")
                                .set("stop-opacity", "0.1"),
                        )
                        .add(Stop::new().set("offset", "100%").set("stop-opacity", "0.1")),
                ),
            )
            .add(
                // Left rectangle (label)
                Rectangle::new()
                    .set("rx", 3)
                    .set("width", total_width)
                    .set("height", 20)
                    .set("fill", "#555"),
            )
            .add(
                // Right rectangle (value)
                Rectangle::new()
                    .set("rx", 3)
                    .set("x", label_width)
                    .set("width", value_width)
                    .set("height", 20)
                    .set("fill", color),
            )
            .add(
                // Separator path
                Path::new()
                    .set("fill", color)
                    .set("d", format!("M{} 0h4v20h-4z", label_width)),
            )
            .add(
                // Label text (shadow)
                Text::new(label_text)
                    .set("x", 30)
                    .set("y", 15)
                    .set("fill", "#010101")
                    .set("fill-opacity", "0.3")
                    .set("text-anchor", "middle")
                    .set("font-family", "DejaVu Sans,Verdana,Geneva,sans-serif")
                    .set("font-size", 11),
            )
            .add(
                Text::new(label_text)
                    .set("x", 30)
                    .set("y", 14)
                    .set("fill", "#fff")
                    .set("text-anchor", "middle")
                    .set("font-family", "DejaVu Sans,Verdana,Geneva,sans-serif")
                    .set("font-size", 11),
            )
            .add(
                Text::new(label_text)
                    .set("x", text_x)
                    .set("y", 15)
                    .set("fill", "#010101")
                    .set("fill-opacity", "0.3")
                    .set("text-anchor", "middle")
                    .set("font-family", "DejaVu Sans,Verdana,Geneva,sans-serif")
                    .set("font-size", 11),
            )
            .add(
                // Value text
                Text::new(full_text)
                    .set("x", text_x)
                    .set("y", 14)
                    .set("fill", "#fff")
                    .set("text-anchor", "middle")
                    .set("font-family", "DejaVu Sans,Verdana,Geneva,sans-serif")
                    .set("font-size", 11),
            );

        document.to_string()
    }

    pub fn create_status_badge(
        highest_severity: &Severity,
        total_count: usize,
        format: &str,
    ) -> Option<String> {
        if total_count == 0 {
            return None;
        }
        match format.to_lowercase().as_str() {
            "svg" => Some(Self::generate_svg_badge(highest_severity, total_count)),
            _ => {
                eprintln!(
                    "Warning: Badge format '{}' is not supported. Defaulting to SVG.",
                    format
                );
                Some(Self::generate_svg_badge(highest_severity, total_count))
            }
        }
    }
}
