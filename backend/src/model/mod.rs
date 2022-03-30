use thiserror::Error as ThisError;

mod db;
mod todo;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
