#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod control;
mod geo_canvas;
mod geo_map;
mod geo_points;
mod geo_rest;
mod util;
mod world;

use crate::control::{control, ControlTransform, TU};
use crate::geo_canvas::geometry_as_shape;
use crate::geo_map::{GeoMap, GeoMapResolution};
use crate::geo_rest::{build_rocket, GeoJsonReceiver, GeoJsonSender};
use crate::util::event::{Event, Events};
use geo_types::Geometry;
use geojson::quick_collection;
use nalgebra::Similarity2;
use std::error::Error;
use std::sync::mpsc;
use std::{io, thread};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color;
use tui::text::{Span, Spans};
use tui::widgets::canvas::Canvas;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

fn get_help_paragraph(address: &str, port: u16) -> Paragraph<'static> {
    let help_text = vec![
        Spans::from(Span::raw(
            "Quit [q] Toggle help [h] Nav [←↑→↓] Zoom [PageUp/PageDown]",
        )),
        Spans::from(Span::raw(format!(
            "Upload GeoJson to http://{}:{}/geo",
            address, port
        ))),
    ];
    Paragraph::new(help_text).block(Block::default().title("Help").borders(Borders::ALL))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    let mut transform: ControlTransform = Similarity2::identity();
    let mut show_help: bool = true;
    let mut geometries: Vec<Geometry<TU>> = vec![];

    let (tx, rx): (GeoJsonSender, GeoJsonReceiver) = mpsc::channel();

    let rocket_tx = tx;
    let rocket = build_rocket(rocket_tx);
    let config = rocket.config();
    let address = String::from(&config.address);
    let port = config.port;
    let _watcher = thread::spawn(move || {
        rocket.launch();
    });

    loop {
        if let Ok(geo_json) = rx.try_recv() {
            if let Ok(collection) = quick_collection::<f64>(&geo_json) {
                geometries = collection.iter().cloned().collect();
            }
        }
        terminal.draw(|f| {
            let constraints: Vec<Constraint> = if show_help {
                vec![Constraint::Percentage(90), Constraint::Percentage(10)]
            } else {
                vec![Constraint::Percentage(100), Constraint::Percentage(0)]
            };
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(constraints)
                .split(f.size());
            let world = Canvas::default()
                .block(Block::default().title("World").borders(Borders::ALL))
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    ctx.draw(&GeoMap {
                        resolution: GeoMapResolution::High,
                        color: Color::White,
                        transform,
                    });
                    ctx.layer();
                    for geometry in &geometries {
                        if let Some(shape) = geometry_as_shape(geometry, &transform) {
                            ctx.draw(&shape);
                        }
                    }
                });
            f.render_widget(world, chunks[0]);
            let help = get_help_paragraph(&address, port);
            f.render_widget(help, chunks[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => break,
                Key::Char('h') => show_help = !show_help, // toggle help
                _ => {
                    let control_transform = control(&key);
                    transform = control_transform * transform;
                }
            }
        }
    }
    Ok(())
}
