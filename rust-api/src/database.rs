use rocket::fairing::AdHoc;

use rocket_db_pools::Database;
use rocket_db_pools::mongodb;

#[derive(Database)]
#[database("mongodb")]
pub struct DbConn(mongodb::Client);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database Stage", |rocket| async {
        rocket.attach(DbConn::init())
    })
}