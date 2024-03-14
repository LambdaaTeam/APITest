use mongodb::Database;
use rocket::{http::Status, serde::json::Json, State};

use crate::{
    models::user_model::{CreateUser, PublicUser},
    services
};

#[post("/register", data = "<user>")]
pub async fn register(
    db: &State<Database>,
    user: Json<CreateUser>
) -> Result<Json<PublicUser>, Status> {
    let user = services::user_service::create(db, user.into_inner()).await?;
    Ok(Json(user))
}