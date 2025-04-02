use surrealdb::error as SurrealError;
use surrealdb::Error as SurrealErrorEnum;

pub enum AppError {
    Database(SurrealErrorEnum),
    DbNotInitialized,
    NotFound(String),
    Validation(String),
}

impl From<AppError> for SurrealErrorEnum {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Database(e) => e,
            AppError::DbNotInitialized => SurrealErrorEnum::from(
                surrealdb::error::Db::Unreachable("DB not initialized".to_string()),
            ),
            AppError::NotFound(e) => {
                SurrealErrorEnum::Api(SurrealError::Api::Query(format!("Entity not found: {}", e)))
            }
            AppError::Validation(e) => {
                SurrealErrorEnum::Api(SurrealError::Api::Query(format!("Validation error: {}", e)))
            }
        }
    }
}

impl From<SurrealErrorEnum> for AppError {
    fn from(err: SurrealErrorEnum) -> Self {
        AppError::Database(err)
    }
}

impl std::fmt::Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {:?}", e),
            Self::DbNotInitialized => write!(f, "Database not initialized"),
            Self::NotFound(e) => write!(f, "Entity not found: {}", e),
            Self::Validation(e) => write!(f, "Validation error: {}", e),
        }
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
