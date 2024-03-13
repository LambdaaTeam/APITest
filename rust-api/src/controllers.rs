#[get("/")]
pub fn healt_check() -> &'static str {
    "OK"
}