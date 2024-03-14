use mongodb::Database;
use rocket::{http::Status, State, serde::json::Json};

use crate::{
    models::user_model::{CreateUser, LoginUser, PublicUser},
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

#[post("/login", data = "<user>")]
pub async fn login(
    db: &State<Database>,
    user: Json<LoginUser>
) -> Result<Json<PublicUser>, Status> {
    let user = services::user_service::login(db, user.into_inner()).await?;
    Ok(Json(user))
}
