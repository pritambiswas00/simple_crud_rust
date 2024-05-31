use serde::{ Deserialize, Serialize };
use sqlx::FromRow;
use crate::todos::model::ToDo;


#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct ToDoView {
    pub id:i32,
    pub title:String,
    pub description:Option<String>,
    pub completed:bool
}


impl  From<ToDo> for ToDoView {
    fn from(value: ToDo) -> Self {
        Self {
            id: value.id,
            title:value.title,
            description: value.description,
            completed: value.completed
        }
    }
}


#[derive(FromRow, Deserialize, Serialize)]
pub struct NewToDoRequest {
    pub title:String,
    pub description: Option<String>,
    pub completed: bool
}


#[derive(FromRow, Deserialize, Debug, Serialize)]
pub struct  UpdateToDoRequest {
    pub id:i32,
    pub title:Option<String>,
    pub description:Option<String>,
    pub completed:Option<bool>
}