use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftTab {
    stroke_color: Color,
}

impl LeftTab {
    pub fn set_stroke_color(&mut self, color: Color) {
        self.stroke_color = color;
    }
}

impl Default for LeftTab {
    fn default() -> Self {
        Self {
            stroke_color: Color::White,
        }
    }
}

impl Widget for LeftTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_style(Style::new().fg(self.stroke_color))
            .title("Top Right Block")
            .render(area, buf);
    }
}
