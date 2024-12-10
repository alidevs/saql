use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, HighlightSpacing, List, ListState, StatefulWidget},
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

impl StatefulWidget for LeftTab {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::bordered()
            .border_style(Style::new().fg(self.stroke_color))
            .title("Left Block");

        let items = ["Item 1", "Item 2", "Item 3", "Item 4"];

        List::new(items)
            .block(block)
            .highlight_symbol(">>")
            .highlight_spacing(HighlightSpacing::Always)
            .render(area, buf, state);
    }
}
