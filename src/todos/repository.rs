use sqlx::{ PgPool, Error };
use crate::todos::model::ToDo;

use super::views::{ NewToDoRequest, UpdateToDoRequest };

pub async fn create(pool: PgPool, new_todos: NewToDoRequest, user_id: i32) -> Result<ToDo, Error> {
    let new_todo = sqlx
        ::query_as::<_, ToDo>(
            "
        INSERT INTO
            todos
            (
                title,
                description,
                completed,
                user_id
            )
        VALUES (
            $1,
            $2,
            $3,
            $4
        )
        RETURNING
            *
    "
        )
        .bind(new_todos.title)
        .bind(new_todos.description)
        .bind(new_todos.completed)
        .bind(user_id)
        .fetch_one(&pool).await?;

    Ok(new_todo)
}

pub async fn update(pool: PgPool, update_todo: UpdateToDoRequest) -> Result<ToDo, Error> {
    let updated_todo = sqlx
        ::query_as::<_, ToDo>(
            "
            UPDATE todos
            SET
                title = $1,
                description = $2,
                completed = $3
            WHERE id = $4
            RETURNING *;
            "
        )
        .bind(update_todo.title)
        .bind(update_todo.description)
        .bind(update_todo.completed)
        .bind(update_todo.id)
        .fetch_one(&pool).await?;
    Ok(updated_todo)
}

pub async fn find_one(pool: PgPool, id: i32) -> Result<ToDo, Error> {
    let todo = sqlx
        ::query_as::<_, ToDo>("
        SELECT * FROM todos WHERE id=$1;
    ")
        .bind(id)
        .fetch_one(&pool).await?;

    Ok(todo)
}
pub async fn find_all(
    pool: PgPool,
    page: i32,
    page_size: i32,
    user_id: i32
) -> Result<Vec<ToDo>, Error> {
    let off_set = (page - 1) * page_size;
    let todos = sqlx
        ::query_as::<_, ToDo>(
            "
    SELECT
        *
        FROM
            todos
        WHERE
            user_id = $1
        ORDER BY
            created_at DESC
        OFFSET
            $2
        LIMIT
            $3
    "
        )
        .bind(user_id)
        .bind(off_set)
        .bind(page_size)
        .fetch_all(&pool).await?;

    Ok(todos)
}

pub async fn delete_todo(pool: PgPool, id: i32) -> Result<ToDo, Error> {
    let deleted_todo = sqlx
        ::query_as::<_, ToDo>("
            DELETE FROM todos WHERE id=$1;
        ")
        .bind(id)
        .fetch_one(&pool).await?;

    Ok(deleted_todo)
}
