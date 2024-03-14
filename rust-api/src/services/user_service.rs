use mongodb::Database;
use rocket::http::Status;

use crate::models::user_model::{CreateUser, PublicUser, User};

pub async fn create(db: &Database, user: CreateUser) -> Result<PublicUser, Status> {
    let collection = db.collection("users");
    let user = User::create(user);

    let result = collection.insert_one(user.clone(), None).await;

    match result {
        Ok(_) => Ok(PublicUser::from(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}