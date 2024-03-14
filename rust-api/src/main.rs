use std::env;

#[macro_use] extern crate rocket;

mod controllers;
mod models;
mod services;
mod database;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir().unwrap().parent().unwrap().join(".env");
    dotenv::from_path(path).ok();

    let db = database::init().await.unwrap();

    rocket::build()
        .mount("/", routes![controllers::healt_check])
        .mount("/v1", routes![
            controllers::auth_controller::register,
        ])
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
