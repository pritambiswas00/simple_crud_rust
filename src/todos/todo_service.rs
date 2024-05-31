use sqlx::PgPool;
use crate::todos;
use super::views::{ NewToDoRequest, UpdateToDoRequest };

pub async fn create_todo(
    pool: PgPool,
    request_new_todo: NewToDoRequest,
    user_id:i32
) -> Result<todos::model::ToDo, sqlx::Error> {
    todos::repository::create(pool, request_new_todo, user_id).await
}

pub async fn update_todo(
    pool: PgPool,
    request_update_todo: UpdateToDoRequest
) -> Result<todos::model::ToDo, sqlx::Error> {
    todos::repository::update(pool, request_update_todo).await
}

pub async fn get_todo_by_id(pool: PgPool, id: i32) -> Result<todos::model::ToDo, sqlx::Error> {
    todos::repository::find_one(pool, id).await
}

pub async fn get_all_todo_by_user_id(
    pool: PgPool,
    page: i32,
    page_size: i32,
    user_id: i32
) -> Result<Vec<todos::model::ToDo>, sqlx::Error> {
    todos::repository::find_all(pool, page, page_size, user_id).await
}

pub async fn delete_todo_by_id(pool: PgPool, id: i32) -> Result<todos::model::ToDo, sqlx::Error> {
    todos::repository::delete_todo(pool, id).await
}
