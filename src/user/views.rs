use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
use sqlx::FromRow;

use crate::user::model::User;

#[derive(FromRow, Debug, Deserialize, Serialize, ToSchema)]
pub struct UserView {
    pub id:i32,
    #[schema(example="user@example.com")]
    pub email:String,
    pub user_name:String,
}

impl  From<User> for UserView {
    fn from(value: User) -> Self {
        Self {
            id:value.id,
            email:value.email,
            user_name: value.user_name
        }
    }
}

#[derive(FromRow, Deserialize, Serialize, ToSchema)]
pub struct  NewUserRequest {
    #[schema(example="user@example.com")]
    pub email:String,
    pub user_name:String,
    pub password:String
}


