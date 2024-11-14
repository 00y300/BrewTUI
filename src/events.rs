use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::App;

pub fn handle_crossterm_events(app: &mut App) -> Result<()> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => on_key_event(app, key),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
        _ => {}
    }
    Ok(())
}

fn on_key_event(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q'))
        | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => app.quit(),
        _ => {}
    }
}
