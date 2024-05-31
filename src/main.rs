mod server;
mod routes;


#[tokio::main]
async fn main() {
    server::boot_strap().await;
}
