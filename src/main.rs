use std::error::Error;
use std::io;
use termion::{raw::IntoRawMode, input::MouseTerminal, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Map, MapResolution};
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    for _ in 1..400 {

    terminal.draw(|f| {
        let world = Canvas::default()
            .block(Block::default().title("Canvas").borders(Borders::ALL))
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&Map {
                    resolution: MapResolution::High,
                    color: Color::White,
                });
            });
        let size = f.size();
        f.render_widget(world, size);
    })?;
    }
    Ok(())
}
