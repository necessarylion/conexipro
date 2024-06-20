use actix_web::{
  body::MessageBody,
  dev::{ServiceRequest, ServiceResponse},
  Error,
};
use actix_web_lab::middleware::Next;

pub async fn handler(
  req: ServiceRequest,
  next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  // pre-processing
  let headers = req.headers();
  let token = headers.get("authorization").unwrap().to_str().unwrap();

  println!("auth middleware: {}", token);

  next.call(req).await
}
