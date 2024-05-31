pub mod authentication {
    pub mod handlers;
    pub mod models;
    pub mod utils;
    pub mod views;
}

pub mod user {
    pub mod model;
    pub mod user_service;
    pub mod repository;
    pub mod views;
    pub mod handlers;
}

pub mod common {
    pub mod errors;
    pub mod middleware;
    pub mod password_encoder;
    pub mod pagination;
}

pub mod database {
    pub mod db;
}

pub mod todos {
    pub mod handlers;
    pub mod model;
    pub mod repository;
    pub mod todo_service;
    pub mod views;
}