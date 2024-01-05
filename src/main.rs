use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let port = std::env::var("PORT").expect("miss PORT");
    let router = Router::new().get(hello);
    let acceptor = TcpListener::new(format!("127.0.0.1:{port}")).bind().await;
    Server::new(acceptor).serve(router).await;
}