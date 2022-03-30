use thiserror::Error as ThisError;

mod db;
mod todo;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}]    ")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
