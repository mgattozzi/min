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
  routes: HashMap<Method, HashMap<Cow<'static, str>, Arc<F>>>,
}

impl<F, G> Router<F>
where
  F: Send + Sync + 'static + Fn(Request<Body>) -> G,
  G: Future<Output = Response<Body>> + Send + Sync + 'static,
{
  pub fn new() -> Self {
    Self {
      routes: HashMap::new(),
    }
  }

  pub fn get(mut self, path: impl Into<Cow<'static, str>>, handler: F) -> Self {
    let routes = self.routes.entry(Method::GET).or_insert(HashMap::new());
    routes.insert(path.into(), Arc::new(handler));
    self
  }

  pub fn put(mut self, path: impl Into<Cow<'static, str>>, handler: F) -> Self {
    let routes = self.routes.entry(Method::PUT).or_insert(HashMap::new());
    routes.insert(path.into(), Arc::new(handler));
    self
  }

  pub fn delete(
    mut self,
    path: impl Into<Cow<'static, str>>,
    handler: F,
  ) -> Self {
    let routes = self.routes.entry(Method::DELETE).or_insert(HashMap::new());
    routes.insert(path.into(), Arc::new(handler));
    self
  }

  pub fn post(
    mut self,
    path: impl Into<Cow<'static, str>>,
    handler: F,
  ) -> Self {
    let routes = self.routes.entry(Method::POST).or_insert(HashMap::new());
    routes.insert(path.into(), Arc::new(handler));
    self
  }

  pub fn trace(
    mut self,
    path: impl Into<Cow<'static, str>>,
    handler: F,
  ) -> Self {
    let routes = self.routes.entry(Method::TRACE).or_insert(HashMap::new());
    routes.insert(path.into(), Arc::new(handler));
    self
  }

  pub fn patch(
    mut self,
    path: impl Into<Cow<'static, str>>,
    handler: F,
  ) -> Self {
    let routes = self.routes.entry(Method::PATCH).or_insert(HashMap::new());
    routes.insert(path.into(), Arc::new(handler));
    self
  }

  pub fn options(
    mut self,
    path: impl Into<Cow<'static, str>>,
    handler: F,
  ) -> Self {
    let routes = self.routes.entry(Method::OPTIONS).or_insert(HashMap::new());
    routes.insert(path.into(), Arc::new(handler));
    self
  }

  pub fn head(
    mut self,
    path: impl Into<Cow<'static, str>>,
    handler: F,
  ) -> Self {
    let routes = self.routes.entry(Method::HEAD).or_insert(HashMap::new());
    routes.insert(path.into(), Arc::new(handler));
    self
  }

  fn find_handler(&self, req: &Request<Body>) -> Option<Arc<F>> {
    self
      .routes
      .get(req.method())
      .and_then(|r| r.get(req.uri().path()).map(|h| h.clone()))
  }

  pub async fn serve(self) -> Result<(), hyper::Error> {
    let router = Arc::new(self);
    let service_fn = make_service_fn(|_| {
      let router = router.clone();
      async move {
        Ok::<_, hyper::Error>(service_fn(move |req| {
          let value = router.find_handler(&req).unwrap()(req);
          async { Ok::<_, hyper::Error>(value.await) }
        }))
      }
    });
    Server::bind(&([0, 0, 0, 0], 8080).into())
      .serve(service_fn)
      .await
  }
}
