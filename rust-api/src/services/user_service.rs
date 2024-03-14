use mongodb::{bson::doc, Database};
use rocket::http::Status;

use crate::models::user_model::{CreateUser, LoginUser, PublicUser, User};

use super::password_service;

pub async fn create(db: &Database, user: CreateUser) -> Result<PublicUser, Status> {
    let collection = db.collection::<User>("users");
    let user = User::create(user);

    let result = collection.insert_one(user.clone(), None).await;

    match result {
        Ok(_) => Ok(PublicUser::from(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn login(db: &Database, login_user: LoginUser) -> Result<PublicUser, Status> {
    let collection = db.collection::<User>("users");

    let result = collection.find_one(doc! { "email": login_user.email}, None).await;

    match result {
        Ok(Some(user)) => {
            let hash = user.password.bytes.clone();

            if password_service::verify_password(&hash, &login_user.password) {
                Ok(PublicUser::from(user))
            } else {
                Err(Status::Unauthorized)
            }
        },
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}