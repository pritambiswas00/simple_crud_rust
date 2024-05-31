use std::fmt::{ Display, Formatter };

use axum::{
    http::StatusCode,
    response::{ IntoResponse, Response },
    Json,
};

use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Serialize, Error, Debug, ToSchema)]
#[serde(rename_all="camelCase")]
pub struct ApiError {
    status_code:u16,
    errors:Vec<String>,
}

impl  Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Error {} ", &self.status_code))
    }
}

impl ApiError {
    pub fn new(status_code:u16, error:String)->Self {
        let errors = vec![error];
        ApiError {
            status_code,
            errors
        }
    }
    pub fn new_internal_error(error:String)->Self {
        let errors = vec![error];
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            errors
        }
    }
    
    pub fn new_bad_request_error(error:String)->Self {
        let errors = vec![error];
        ApiError {
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            errors
        }
    }
    
    pub fn new_unauthorized_error(error:String)->Self {
        let errors = vec![error];
        ApiError {
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
            errors
        }
    }
    
    pub fn new_not_found_error(error:String)->Self {
        let errors = vec![error];
        ApiError {
            status_code: StatusCode::NOT_FOUND.as_u16(),
            errors
        }
    }
    
    pub fn new_conflict_error(error:String)->Self {
        let errors = vec![error];
        ApiError {
            status_code: StatusCode::CONFLICT.as_u16(),
            errors
        }
    }
    
    pub fn append_error(&mut self , error:String) {
        self.errors.push(error);
    }
}

impl  IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status_code).unwrap(), Json(self)).into_response()
    }
}

