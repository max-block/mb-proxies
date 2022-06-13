mod app;
pub mod db;
mod error;
mod service;

pub use app::App;
pub use db::model::*;
pub use error::AppError;
pub use error::Result;
pub use mongodb::bson::doc;
pub use mongodb::error::Error as MongoError;
