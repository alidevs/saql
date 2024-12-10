use ratatui::style::Color;

use crate::blocks::{BottomRightTab, LeftTab, TopRightTab};

#[derive(Debug, Default)]
pub struct TabManager {
    pub current_tab: TabKind,
    pub left_tab: LeftTab,
    pub top_right_tab: TopRightTab,
    pub bottom_right_tab: BottomRightTab,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum TabKind {
    #[default]
    Left,
    TopRight,
    BottomRight,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            current_tab: TabKind::Left,
            left_tab: LeftTab::default(),
            top_right_tab: TopRightTab::default(),
            bottom_right_tab: BottomRightTab::default(),
        }
    }

    pub fn next(&mut self) {
        self.current_tab = match self.current_tab {
            TabKind::Left => TabKind::TopRight,
            TabKind::TopRight => TabKind::BottomRight,
            TabKind::BottomRight => TabKind::Left,
        }
    }

    pub fn prev(&mut self) {
        self.current_tab = match self.current_tab {
            TabKind::TopRight => TabKind::Left,
            TabKind::BottomRight => TabKind::TopRight,
            TabKind::Left => TabKind::BottomRight,
        };
    }

    pub fn get_color(&self, tab: TabKind) -> Color {
        if self.current_tab == tab {
            Color::Red
        } else {
            Color::White
        }
    }

    pub fn update_colors(&mut self) {
        self.left_tab
            .set_stroke_color(self.get_color(TabKind::Left));
        self.top_right_tab
            .set_stroke_color(self.get_color(TabKind::TopRight));
        self.bottom_right_tab
            .set_stroke_color(self.get_color(TabKind::BottomRight));
    }
}
