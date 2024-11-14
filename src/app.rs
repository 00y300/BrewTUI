use color_eyre::Result;
use ratatui::DefaultTerminal;

use crate::{events::handle_crossterm_events, ui::draw};

#[derive(Debug, Default)]
pub struct App {
    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(draw)?;
            handle_crossterm_events(&mut self)?;
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
