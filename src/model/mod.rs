use thiserror::Error as ThisError;

mod db;
mod todo;
mod user;

// re-export
pub use db::init_db;
pub use db::Db;
pub use todo::{Todo, TodoMac, TodoPatch, TodoStatus};

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}]    ")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Username is already taken - {0}")]
    UserNameIsAlreadyTaken(String),

    #[error("Invalid password for username - {0}")]
    InvalidPassword(String),
}

pub fn handle_fetch_one_result<T>(
    result: Result<T, sqlx::Error>,
    typ: &'static str,
    data: String,
) -> Result<T, Error> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => Error::EntityNotFound(typ, data),
        other => Error::SqlxError(other),
    })
}
