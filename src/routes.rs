use axum::{
    middleware::{self},
    routing::{get, post, patch, delete},
    Router,
};
use simple_crud_rust::{self, authentication, common, todos::{self}, user};
use sqlx::{Pool, Postgres};

pub fn build_routes(pool: Pool<Postgres>) -> Router {
    let auth_state = common::middleware::AuthenticationState {
        pool,
        jwt_secret: authentication::models::JWT_SECRET.to_string(),
    };

    let api_routes = Router::new()
        .nest(
            "/auth",
            Router::new().route("/tokens", post(authentication::handlers::get_tokens)),
        )
        .nest("/todos", Router::new()
                    .route("/create", post(todos::handlers::create_todo))
                    .route("/update", patch(todos::handlers::update_todo))
                    .route("/find_one", get(todos::handlers::get_todo))
                    .route("/find_all", get(todos::handlers::get_all_todos))
                    .route("/delete", delete(todos::handlers::delete_todo))
        ).route_layer(middleware::from_fn_with_state(auth_state.clone(), common::middleware::authentication_middleware))
        .nest(
            "/users",
            Router::new()
                .route(
                    "/",
                    get(user::handlers::find_user_by_email).route_layer(
                        middleware::from_fn_with_state(auth_state.clone(), common::middleware::authentication_middleware),
                    ),
                )
                .route("/", post(user::handlers::register_user)),
        );

    Router::new()
        .nest("/api", api_routes.with_state(auth_state))
}