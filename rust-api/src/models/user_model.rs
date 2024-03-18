use chrono::{DateTime, Utc};
use mongodb::bson::{oid, spec::BinarySubtype, Binary};
use serde::{Deserialize, Serialize};

use crate::services::password_service::hash_password;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: oid::ObjectId,
    pub name: String,
    pub email: String,
    pub password: Binary,
    pub schema_version: u8,
    pub create_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub create_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicUserWithToken {
    pub id: String,
    pub name: String,
    pub email: String,
    pub create_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        PublicUser {
            id: user.id.to_hex(),
            name: user.name,
            email: user.email,
            create_at: user.create_at,
            updated_at: user.updated_at,
        }
    }
}

impl User {
    pub fn new(user: CreateUser) -> Self {
        let hash = hash_password(&user.password);
        User {
            id: oid::ObjectId::new(),
            name: user.name,
            email: user.email,
            password: Binary {
                subtype: BinarySubtype::Generic,
                bytes: hash,
            },
            schema_version: 1,
            create_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl PublicUserWithToken {
    pub fn new(user: User, token: String) -> Self {
        PublicUserWithToken {
            id: user.id.to_hex(),
            name: user.name,
            email: user.email,
            create_at: user.create_at,
            updated_at: user.updated_at,
            token,
        }
    }
}
