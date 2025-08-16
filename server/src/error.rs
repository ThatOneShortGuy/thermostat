use axum::response::IntoResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    InternalServerError,
    Unauthorized,
    InvalidToken,
    LoginFailed,
    Description(String),
    NotLoggedIn,
    SessionExpired,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let span = tracing::trace_span!("into_response", ?self);
        let _enter = span.enter();
        match self {
            AppError::InternalServerError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            )
                .into_response(),
            AppError::Unauthorized => {
                (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
            }
            AppError::LoginFailed => {
                (axum::http::StatusCode::UNAUTHORIZED, "Login Failed").into_response()
            }
            AppError::Description(d) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", d),
            )
                .into_response(),
            AppError::InvalidToken => {
                (axum::http::StatusCode::UNAUTHORIZED, "Invalid Token").into_response()
            }
            AppError::NotLoggedIn => {
                (axum::http::StatusCode::UNAUTHORIZED, "Not Logged In").into_response()
            }
            AppError::SessionExpired => {
                (axum::http::StatusCode::UNAUTHORIZED, "Session Expired").into_response()
            }
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        tracing::error!(?e, "Database error");
        AppError::InternalServerError
    }
}

#[cfg(debug_assertions)]
impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Description(format!("{}\n{}", e.to_string(), e.backtrace()))
    }
}

#[cfg(not(debug_assertions))]
impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        tracing::warn!(?e, "Error occurred");
        AppError::InternalServerError
    }
}
