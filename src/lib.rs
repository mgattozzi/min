use hyper::{
  service::{
    make_service_fn,
    service_fn,
  },
  Body,
  Method,
  Request,
  Response,
  Server,
};
use std::{
  borrow::Cow,
  collections::HashMap,
  future::Future,
  sync::Arc,
};
#[derive(Debug)]
pub struct Router<F> {
  post_routes: HashMap<Cow<'static, str>, Arc<F>>,
}

impl<F, G> Router<F>
where
  F: Send + Sync + 'static + Fn(Request<Body>) -> G,
  G: Future<Output = Response<Body>> + Send + Sync + 'static,
{
  pub fn new() -> Self {
    Self {
      post_routes: HashMap::new(),
    }
  }

  pub fn post(
    mut self,
    path: impl Into<Cow<'static, str>>,
    handler: F,
  ) -> Self {
    self.post_routes.insert(path.into(), Arc::new(handler));
    self
  }

  fn find_handler(&self, req: &Request<Body>) -> Option<Arc<F>> {
    match (req.method(), req.uri().path()) {
      (&Method::POST, path) => self.post_routes.get(path).map(|h| h.clone()),
      _ => todo!(),
    }
  }

  pub async fn serve(self) -> Result<(), hyper::Error> {
    let service = Arc::new(RouterService::new(self));
    let service_fn = make_service_fn(|_| {
      let service = service.clone();
      async move {
        Ok::<_, hyper::Error>(service_fn(move |req| {
          let value = service.router.find_handler(&req).unwrap()(req);

          async { Ok::<_, hyper::Error>(value.await) }
        }))
      }
    });
    Server::bind(&([0, 0, 0, 0], 8080).into())
      .serve(service_fn)
      .await
  }
}

#[derive(Debug)]
pub struct RouterService<F> {
  pub router: Router<F>,
}

impl<F, G> RouterService<F>
where
  F: Send + Sync + 'static + Fn(Request<Body>) -> G,
  G: Future<Output = Response<Body>> + Send + Sync + 'static,
{
  fn new(router: Router<F>) -> Self {
    Self { router }
  }
}
