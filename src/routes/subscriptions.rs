use axum::extract::rejection::FormRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_macros::FromRequest;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::startup::AppState;

pub async fn subscribe(
    State(state): State<AppState>,
    Form(form): Form<SubscribeForm>,
) -> StatusCode {
    tracing::info!(
        "Adding '{}' '{}' as a new subscriber.",
        form.email,
        form.name
    );
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved");
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
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
