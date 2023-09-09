use derive_more::Display;

#[derive(Debug, Display)]
pub enum RepositoryError {
    #[display(fmt = "Internal Server Error")]
    Internal,
    #[display(fmt = "NotFound")]
    NotFound
}