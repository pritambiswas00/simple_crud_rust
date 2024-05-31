use sqlx::PgPool;

use crate::common::password_encoder;
use crate::user::model::User;
use crate::user::{ repository as user_repo, views::NewUserRequest };

pub async fn register_user(
    pool:PgPool,
    new_user:NewUserRequest
)->Result<User, sqlx::Error>{
    let new_user_request = NewUserRequest {
        password: password_encoder::encode_password(new_user.password.as_str()),
        ..new_user
    };

    user_repo::register_user(pool, new_user_request).await
}

pub async fn find_user_by_email(pool: PgPool, email: &str) -> Result<User, sqlx::Error> {
    user_repo::find_user_by_email(pool, email).await
}

pub async fn find_user_by_id(pool: PgPool, id: i32) -> Result<User, sqlx::Error> {
    user_repo::find_user_by_id(pool, id).await
}

pub async fn login(pool: PgPool, email: &str, password: &str) -> Result<User, sqlx::Error> {
    let user = user_repo::find_user_by_email(pool, email).await?;
    if !password_encoder::verify_password(password, user.password.as_str()) {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(user)
}