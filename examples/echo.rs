use hyper::{
  Body,
  Request,
  Response,
};
use min::Router;
use std::error::Error;
use tracing::{
  error,
  event,
  instrument,
  Level,
};
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
  Router::new()
    .post("/echo", echo)
    .post(String::from("/bye"), echo)
    .serve()
    .await?;

  Ok(())
}

#[instrument]
async fn echo(req: Request<Body>) -> Response<Body> {
  event!(Level::INFO, "We got a request");
  Response::new(req.into_body())
}
