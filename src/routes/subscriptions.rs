use axum::extract::rejection::FormRejection;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use axum_macros::FromRequest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub async fn subscribe(
    Form(data): Form<SubscribeForm>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    Ok((StatusCode::OK, Json(json!(data))))
}

#[derive(Debug)]
pub struct ApiError {
    pub code: StatusCode,
    pub message: String,
}

#[derive(FromRequest)]
#[from_request(via(axum::Form), rejection(ApiError))]
pub struct Form<T>(T);

#[derive(Deserialize, Serialize)]
pub struct SubscribeForm {
    pub name: String,
    pub email: String,
}

impl From<FormRejection> for ApiError {
    fn from(rejection: FormRejection) -> Self {
        let code = match rejection {
            FormRejection::FailedToDeserializeForm(_) => StatusCode::BAD_REQUEST,
            FormRejection::FailedToDeserializeFormBody(_) => StatusCode::BAD_REQUEST,
            FormRejection::InvalidFormContentType(_) => StatusCode::BAD_REQUEST,
            FormRejection::BytesRejection(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            code,
            message: rejection.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "message": self.message,
            "origin": "derive_from_request"
        });

        (self.code, axum::Json(payload)).into_response()
    }
}
