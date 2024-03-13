#[macro_use] extern crate rocket;

mod controllers;
mod database;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![controllers::healt_check])
        .mount("/v1", routes![])
        .attach(database::stage())
}
