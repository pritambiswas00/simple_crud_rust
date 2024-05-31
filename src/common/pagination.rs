use serde::{ Deserialize, Serialize };
use utoipa::{ IntoParams, ToSchema };

#[derive(Deserialize, IntoParams)]
pub struct PaginationParams {
    pub page:Option<i32>,
    pub size: Option<i32>
}

#[derive(ToSchema, Serialize)]
pub struct PaginatedView<T> {
    pub page:i32,
    pub size:i32,
    pub items:Vec<T>
}