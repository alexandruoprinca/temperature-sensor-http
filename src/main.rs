use rand::Rng;
use rocket::serde::json;

#[macro_use]
extern crate rocket;

lazy_static::lazy_static! {
static ref TEMPERATURE: std::sync::Mutex<f32> = std::sync::Mutex::new(generate_random_temperature());
}

fn generate_random_temperature() -> f32 {
    rand::thread_rng().gen_range(-35..35) as f32
}

#[get("/temperature", format = "json")]
fn current_temperature() -> json::Value {
    json::json!(
        {"status": "ok",
            "temperature": TEMPERATURE.lock().unwrap().to_string()})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![current_temperature])
}
