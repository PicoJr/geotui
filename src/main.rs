#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod control;
mod custom_map;
mod geo_canvas;
mod geo_rest;
mod util;
mod world;

use crate::control::{control, ControlTransform, TU};
use crate::custom_map::{CustomMap, CustomMapResolution};
use crate::geo_canvas::geometry_as_shape;
use crate::geo_rest::{rocket, GeoJsonReceiver, GeoJsonSender};
use crate::util::event::{Event, Events};
use geo_types::Geometry;
use geojson::quick_collection;
use geojson::Error::GeoJsonExpectedObject;
use nalgebra::{Similarity2, Vector2};
use std::error::Error;
use std::sync::mpsc;
use std::{io, thread};
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

    let mut transform: ControlTransform = Similarity2::identity();
    let mut geometries: Vec<Geometry<TU>> = vec![];

    let (tx, rx): (GeoJsonSender, GeoJsonReceiver) = mpsc::channel();

    let rocket_tx = tx.clone();
    let watcher = thread::spawn(move || {
        rocket(rocket_tx).launch();
    });

    loop {
        if let Ok(geo_json) = rx.try_recv() {
            if let Ok(collection) = quick_collection::<f64>(&geo_json) {
                geometries = collection.iter().cloned().collect();
            }
        }
        terminal.draw(|f| {
            let world = Canvas::default()
                .block(Block::default().title("Canvas").borders(Borders::ALL))
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    ctx.draw(&CustomMap {
                        resolution: CustomMapResolution::High,
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
            let size = f.size();
            f.render_widget(world, size);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => break,
                _ => {
                    let control_transform = control(&key);
                    transform = transform * control_transform;
                }
            }
        }
    }
    Ok(())
}
