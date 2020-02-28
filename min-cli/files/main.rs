use hyper::{Body, Request, Response};
use min::Router;
use std::error::Error;
use tracing::error;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        error!("Server Failed with: {}", e);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    Router::new().post("/echo", echo).serve().await?;

    Ok(())
}

async fn echo(req: Request<Body>) -> Response<Body> {
    Response::new(req.into_body())
}
