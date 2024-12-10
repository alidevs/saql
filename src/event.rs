use crossterm::event::{Event, KeyCode, KeyEvent};

pub enum AppEvent {
    Quit,
    NextTab,
    PrevTab,
    Up,
    Down,
    None,
}

pub fn handle_event(event: Event) -> AppEvent {
    match event {
        Event::Key(key) => handle_key_event(key),
        _ => AppEvent::None,
    }
}

fn handle_key_event(key: KeyEvent) -> AppEvent {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => AppEvent::Quit,
        KeyCode::Char('h') | KeyCode::Up => AppEvent::PrevTab,
        KeyCode::Char('l') | KeyCode::Down => AppEvent::NextTab,
        KeyCode::Char('j') => AppEvent::Down,
        KeyCode::Char('k') => AppEvent::Up,
        _ => AppEvent::None,
    }
}
