use axum::{ extract::State, Json };
use crate::{authentication::{ models::JWT_SECRET, utils, views::{ DeserializeToken, LoginSchema }}, common::{errors::ApiError, middleware::AuthenticationState}, user::user_service};

static  ACCESS_TOKEN_EXPIRATION:usize = 60*60*1000;
static REFRESH_TOKEN_EXPIRATION:usize = 24 * 60 * 60 * 1000;

//Get Token For the Login Session//
pub async fn get_tokens(
    State(store):State<AuthenticationState>,
    Json(request) : Json<LoginSchema>
)->Result<Json<DeserializeToken>, ApiError> {
    let LoginSchema { email, password } = request;
    let user = user_service::login(store.pool, &email, &password).await;

    match user {
        Ok(user)=>{
            let access_token = utils::encode_token(&user, ACCESS_TOKEN_EXPIRATION, "access_token", JWT_SECRET).unwrap();
            let refresh_token = utils::encode_token(&user, REFRESH_TOKEN_EXPIRATION, "refresh", JWT_SECRET).unwrap();

            Ok(Json(DeserializeToken {
                user_id:user.id,
                access_token,
                refresh_token
            }))
        },
        Err(_)=> Err(ApiError::new_bad_request_error("User not found with given credentials".to_string()))
    }


}
