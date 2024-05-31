use serde::{ Deserialize, Serialize };
use sqlx::FromRow;
use chrono::{ DateTime, Utc};
use std::fmt;
#[derive(Debug,FromRow, Clone, Deserialize, Serialize )]
pub struct  User {
    pub id:i32,
    pub email:String,
    pub password: String,
    pub user_name:String,
    pub created_at:DateTime<Utc>,
    pub updated_at:DateTime<Utc>,
}


impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User {{ id: {}, email: {}, password: {}, user_name: {}, created_at: {}, updated_at: {} }}",
            self.id, self.email, self.password, self.user_name, self.created_at, self.updated_at
        )
    }
}