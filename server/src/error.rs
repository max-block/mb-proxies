use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("app error")]
    AppError(#[from] app::AppError),

    #[error("mongo error")]
    MongoError(#[from] app::MongoError)
}

impl ResponseError for ServerError {}

pub type Result<T> = std::result::Result<T, ServerError>;
