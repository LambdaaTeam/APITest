pub mod auth_controller;

#[get("/")]
pub fn healt_check() -> &'static str {
    "OK"
}