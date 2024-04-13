use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_with::DisplayFromStr;

pub type ApiResult<T, E = ApiError> = Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("Docker error yolo")]
    Dockworker(#[from] dockworker::errors::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(serde::Serialize)]
        struct ErrorResponse<'a> {
            #[serde_as(as = "DisplayFromStr")]
            message: &'a ApiError,
        }

        println!("API error: {self:?}");

        (
            self.status_code(),
            Json(ErrorResponse { message: &self }),
        )
            .into_response()
    }
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        use ApiError::*;

        match self {
            Dockworker(_) | Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
