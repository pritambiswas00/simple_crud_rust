use axum::{ extract::{ Path, Query, State }, Extension, Json };

use crate::{
    common::{
        errors::ApiError,
        middleware::AuthenticationState,
        pagination::{ PaginatedView, PaginationParams },
    },
    todos::todo_service,
    user::model::User,
};
use super::views::{ NewToDoRequest, ToDoView, UpdateToDoRequest };
use tracing::info;

pub async fn create_todo(
    State(state): State<AuthenticationState>,
    Extension(user): Extension<User>,
    Json(request): Json<NewToDoRequest>,
) -> Result<Json<ToDoView>, ApiError> {
    info!("Created ToDo Handler Called");
    let new_todo = todo_service::create_todo(state.pool, request, user.id).await;
    match new_todo {
        Ok(new_todo) => Ok(Json(ToDoView::from(new_todo))),
        Err(error) =>
            match &error {
                sqlx::Error::Database(db_err) => {
                    if db_err.constraint().is_some() {
                        Err(ApiError::new_conflict_error("ToDo already exists".to_string()))
                    } else {
                        Err(ApiError::new_internal_error(error.to_string()))
                    }
                }
                _ => Err(ApiError::new_internal_error(error.to_string())),
            }
    }
}

pub async fn update_todo(
    State(state): State<AuthenticationState>,
    Json(request): Json<UpdateToDoRequest>
    
) -> Result<Json<ToDoView>, ApiError> {
    let updated_todo = todo_service::update_todo(state.pool, request).await;
    match updated_todo {
        Ok(updated_todo) => Ok(Json(ToDoView::from(updated_todo))),
        Err(error) =>
            match &error {
                sqlx::Error::Database(db_error) => {
                    if db_error.constraint().is_some() {
                        Err(ApiError::new_conflict_error("Couldnt update the todo".to_string()))
                    } else {
                        Err(ApiError::new_internal_error(error.to_string()))
                    }
                }
                _ => Err(ApiError::new_internal_error(error.to_string())),
            }
    }
}

pub async fn get_todo(
    State(state): State<AuthenticationState>,
    Path(id): Path<i32>
) -> Result<Json<ToDoView>, ApiError> {
    let todo = todo_service::get_todo_by_id(state.pool, id).await;
    match todo {
        Ok(todo) => Ok(Json(ToDoView::from(todo))),
        Err(_) => Err(ApiError::new_not_found_error("ToDo with Id not found".to_string())),
    }
}

pub async fn get_all_todos(
    State(state): State<AuthenticationState>,
    Query(query): Query<PaginationParams>,
    Extension(user):Extension<User>
) -> Result<Json<PaginatedView<ToDoView>>, ApiError> {
    let (page, size) = (query.page.unwrap_or(1), query.size.unwrap_or(10));
    let todos = todo_service::get_all_todo_by_user_id(state.pool, page, size, user.id).await;
    match todos {
        Ok(todos) =>
            Ok(
                Json(PaginatedView {
                    page,
                    size,
                    items: todos.into_iter().map(ToDoView::from).collect(),
                })
            ),
        Err(error) => Err(ApiError::new_internal_error(error.to_string())),
    }
}

pub async fn delete_todo(
    State(state): State<AuthenticationState>,
    Path(id): Path<i32>
) -> Result<Json<ToDoView>, ApiError> {
    let deleted_todo = todo_service::delete_todo_by_id(state.pool, id).await;
    match deleted_todo {
        Ok(deleted_todo) => Ok(Json(ToDoView::from(deleted_todo))),
        Err(error) => Err(ApiError::new_internal_error(error.to_string())),
    }
}
