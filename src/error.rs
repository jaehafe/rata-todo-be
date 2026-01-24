use axum::http::StatusCode;
use axum::response::IntoResponse;
use diesel::result::Error as DieselError;
use diesel_async::pooled_connection::deadpool::PoolError;

pub type AppResult<T> = Result<T, AppError>;

pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        }.into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::Internal(err.into()),
        }
    }
}

impl From<PoolError> for AppError {
    fn from(err: PoolError) -> Self {
        AppError::Internal(err.into())
    }
}
