use axum::{ body::Body, extract::{ Request, State }, middleware::Next, response::Response };
use sqlx::{ Pool, Postgres };
use crate::{ authentication::utils::validate_token, user::user_service };
use super::errors::ApiError;

pub struct AuthenticationState {
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}

impl Clone for AuthenticationState {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            jwt_secret: self.jwt_secret.clone(),
        }
    }
}

pub async fn authentication_middleware(
    State(AuthenticationState { pool, jwt_secret }): State<AuthenticationState>,
    mut req: Request<Body>,
    next: Next
) -> Result<Response, ApiError> {
    let token_from_header = match req.headers().get("Authorization") {
        Some(token) => token,
        None => {
            return Err(ApiError::new_unauthorized_error("Token not Found".to_string()));
        }
    };

    let bearer_token = match token_from_header.to_str() {
        Ok(token) => token,
        Err(_) => {
            return Err(ApiError::new_unauthorized_error("Token is not valid".to_string()));
        }
    };

    let token = bearer_token.replace("Bearer", "");
    let claims = match validate_token(token.as_str(), &jwt_secret) {
        Ok(claims) => claims,
        Err(_) => {
            return Err(ApiError::new_unauthorized_error("User not found".to_string()));
        }
    };

    let user = user_service::find_user_by_id(pool, claims.user_id).await;

    match user {
        Ok(user) => {
            req.extensions_mut().insert(user);
        }
        Err(_) => {
            return Err(ApiError::new_unauthorized_error("User not found".to_string()));
        }
    }

    Ok(next.run(req).await)
}
