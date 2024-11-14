use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame) {
    let title = Line::from("Ratatui Simple Template")
        .bold()
        .blue()
        .centered();
    let text = "Hello, Ratatui!\n\n\
        Created using https://github.com/ratatui/templates\n\
        Press `Esc`, `Ctrl-C` or `q` to stop running.";
    frame.render_widget(
        Paragraph::new(text)
            .block(Block::bordered().title(title))
            .centered(),
        frame.area(),
    )
}
