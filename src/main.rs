mod app;
mod blocks;
mod event;
mod tab_manager;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = App::new().run(&mut terminal);

    ratatui::restore();

    app_result
}
