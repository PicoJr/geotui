use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use geojson::GeoJson;

pub(crate) type GeoJsonSender = Sender<GeoJson>;
pub(crate) type GeoJsonReceiver = Receiver<GeoJson>;

#[post("/polygon", format = "json", data = "<message>")]
fn new(message: Json<GeoJson>, map: State<Mutex<GeoJsonSender>>) -> JsonValue {
    match map.lock().expect("lock").send(message.into_inner()) {
        Ok(()) => json!({ "status": "ok" }),
        Err(_) => json!({
            "status": "error",
            "reason": "ID exists. Try put."
        }),
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub(crate) fn rocket(tx: GeoJsonSender) -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![new])
        .register(catchers![not_found])
        .manage(Mutex::new(tx))
}
