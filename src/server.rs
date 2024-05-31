use std::net::{ Ipv4Addr, SocketAddr };
use simple_crud_rust::database::{self, db::get_postgres_pool};
use tracing::info;
use tokio::net::TcpListener;
use crate::routes::build_routes;



pub async fn boot_strap(){
    
    tracing_subscriber::fmt::init();

    let db_uri = dotenv::var("DATABASE_URL").expect("DATABASE_URL required");

    let pool =  get_postgres_pool(db_uri.as_str()).await.unwrap_or_else(|_|{
        panic!(
            "Failed to connect to Postgres with provided URL {}", db_uri
        )
    });
 
     database::db::migrate(&pool).await;

    let router = build_routes(pool);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 5000));
    let tcp_listener = TcpListener::bind(&address).await.unwrap();
    info!("Listening on {}", address);
    axum::serve(tcp_listener, router.into_make_service())
                .await
                .unwrap()
}



