use derive_more::Display;
use crate::application::repositories::error::RepositoryError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {_0}")]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "NotFound")]
    NotFound,
}

impl From<RepositoryError> for ServiceError {
    fn from(error: RepositoryError) -> ServiceError {
        match error {
            RepositoryError::NotFound => ServiceError::NotFound,
            RepositoryError::Internal => ServiceError::InternalServerError,
        }
    }
}