use serde::{ Deserialize, Serialize };
use crate::user::model::User;

pub static JWT_SECRET:&str = "some_secret_key";

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub sub:String,
    pub user_id:i32,
    pub token_type:String,
    pub expire:usize
}

impl Token {
    pub fn from_user(user:&User, token_type:&str, expire:usize)->Self {
        Token {
            sub:user.user_name.clone(),
            user_id: user.id,
            token_type:token_type.to_string(),
            expire
        }
    }
}


