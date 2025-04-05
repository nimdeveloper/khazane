use surrealdb::Error as SurrealDbError;

#[derive(Debug)]
pub enum AppError {
    Database(SurrealDbError),
    DbNotInitialized,
    NotFound(String),
    Validation(String),
}

impl From<AppError> for SurrealDbError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Database(e) => e,
            AppError::DbNotInitialized => SurrealDbError::Api(surrealdb::error::Api::Query(
                "Database not initialized".to_string(),
            )),
            AppError::NotFound(e) => SurrealDbError::Api(surrealdb::error::Api::Query(format!(
                "Entity not found: {}",
                e
            ))),
            AppError::Validation(e) => SurrealDbError::Api(surrealdb::error::Api::Query(format!(
                "Validation error: {}",
                e
            ))),
        }
    }
}

impl From<SurrealDbError> for AppError {
    fn from(err: SurrealDbError) -> Self {
        AppError::Database(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::DbNotInitialized => write!(f, "Database not initialized"),
            Self::NotFound(e) => write!(f, "Entity not found: {}", e),
            Self::Validation(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
