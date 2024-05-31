use serde::{ Deserialize, Serialize };
use sqlx::FromRow;
use chrono::{ DateTime, Utc };


#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct ToDo {
    pub id:i32,
    pub title:String,
    pub description:Option<String>,
    pub completed:bool,
    pub user_id:i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}