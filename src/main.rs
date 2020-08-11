mod custom_map;
mod util;
mod world;

use crate::custom_map::{CustomMap, CustomMapResolution};
use crate::util::event::{Event, Events};
use nalgebra::Similarity2;
use std::error::Error;
use std::io;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::style::Color;
use tui::widgets::canvas::Canvas;
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    let mut scale: f64 = 1.0;

    loop {
        terminal.draw(|f| {
            let world = Canvas::default()
                .block(Block::default().title("Canvas").borders(Borders::ALL))
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    ctx.draw(&CustomMap {
                        resolution: CustomMapResolution::High,
                        color: Color::White,
                        transform: Similarity2::identity().append_scaling(scale),
                    });
                });
            let size = f.size();
            f.render_widget(world, size);
        })?;

        if let Event::Input(key) = events.next()? {
            if key == Key::Char('q') {
                break;
            }
            if key == Key::PageUp {
                scale = scale * 2.0;
            }
            if key == Key::PageDown {
                scale = scale / 2.0;
            }
        }
    }
    Ok(())
}
