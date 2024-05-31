use axum::{
    extract::{Query, State},
    Json,
};
use utoipa::IntoParams;
use tracing::info;

use crate::{
    common::{errors::ApiError, middleware::AuthenticationState},
    user::{
        user_service,
        views::{NewUserRequest, UserView},
    },
};


pub async fn register_user(
    State(state): State<AuthenticationState>,
    Json(request): Json<NewUserRequest>
) -> Result<Json<UserView>, ApiError> {
    info!("Register Handler Called");
    let user = user_service::register_user(state.pool, request).await;
    match user {
        Ok(user) => Ok(Json(UserView::from(user))),
        Err(error) =>
            match &error {
                sqlx::Error::Database(db_err) => {
                    if db_err.constraint().is_some() {
                        Err(
                            ApiError::new_conflict_error(
                                "User with this email already exists".to_string()
                            )
                        )
                    } else {
                        Err(ApiError::new_internal_error(error.to_string()))
                    }
                }
                _ => Err(ApiError::new_internal_error(error.to_string())),
            }
    }
}

#[derive(serde::Deserialize, IntoParams)]
pub struct FindUserQuery {
    pub email: Option<String>,
}

pub async fn find_user_by_email(
    State(state): State<AuthenticationState>,
    Query(query): Query<FindUserQuery>
) -> Result<Json<UserView>, ApiError> {
    let email = query.email;

    let email = match email {
        Some(email) => email,
        None => {
            return Err(ApiError::new_bad_request_error("`email` query is required".to_string()));
        }
    };

    let user = user_service::find_user_by_email(state.pool, email.as_str()).await;

    match user {
        Ok(user) => Ok(Json(UserView::from(user))),
        Err(_) => Err(ApiError::new_not_found_error(format!("User with email {} not found", email))),
    }
}
