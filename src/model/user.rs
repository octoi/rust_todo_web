use serde::{Deserialize, Serialize};
use sqlb::HasFields;

use super::{handle_fetch_one_result, Db};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(sqlb::Fields, Debug, Default, Clone, Deserialize)]
pub struct UserPatch {
    pub username: String,
    pub password: String,
}

pub struct UserMac;

impl UserMac {
    const TABLE: &'static str = "person";
    const COLUMNS: &'static [&'static str] = &["id", "username", "password"];
}

impl UserMac {
    pub async fn register(db: &Db, data: UserPatch) -> Result<User, super::Error> {
        // TODO: hash password

        let cloned = data.clone();
        let fields = data.fields();

        let username = &cloned.username;
        let user = Self::get(db, &username).await;

        // If user exist
        if let Ok(_) = user {
            return Err(super::Error::UserNameIsAlreadyTaken(username.to_string()));
        }

        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);

        let result = sb.fetch_one(db).await;

        super::handle_fetch_one_result(result, Self::TABLE, cloned.username)
    }

    pub async fn login(db: &Db, data: UserPatch) -> Result<User, super::Error> {
        let username = data.username.clone();
        let user = Self::get(db, &username).await?;

        if user.username == data.username.clone() && user.password == data.password.clone() {
            Ok(user)
        } else {
            Err(super::Error::InvalidPassword(username))
        }
    }

    pub async fn get(db: &Db, username: &str) -> Result<User, super::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("username", username);

        let result = sb.fetch_one(db).await;

        handle_fetch_one_result(result, Self::TABLE, username.to_string())
    }
}

#[cfg(test)]
#[path = "../_tests/model_user.rs"]
mod tests;
