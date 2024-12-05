use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use std::io;

use crate::{
    event::{handle_event, AppEvent},
    tab_manager::TabManager,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

pub struct App {
    mode: Mode,
    tab_manager: TabManager,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::Running,
            tab_manager: TabManager::new(),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<impl ratatui::backend::Backend>,
    ) -> io::Result<()> {
        while self.mode != Mode::Quit {
            terminal.draw(|frame| {
                self.tab_manager.update_colors();

                let [left, right] = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(25), Constraint::Fill(1)])
                    .areas(frame.area());

                let [top, bottom] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Fill(1); 2])
                    .areas(right);

                frame.render_widget(self.tab_manager.left_tab, left);
                frame.render_widget(self.tab_manager.top_right_tab, top);
                frame.render_widget(self.tab_manager.bottom_right_tab, bottom);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let event = handle_event(Event::Key(key));
                    self.handle_event(event);
                }
            }
        }

        Ok(())
    }
    fn handle_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Quit => self.mode = Mode::Quit,
            AppEvent::NextTab => self.tab_manager.next(),
            AppEvent::PrevTab => self.tab_manager.prev(),
            AppEvent::None => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
