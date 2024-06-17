use actix_http::h1::Payload;
use actix_web::{
  body::MessageBody,
  dev::{ServiceRequest, ServiceResponse},
  Error,
};
use actix_web_lab::middleware::Next;
use serde_json::{json, Value};

pub async fn my_mw(
  body: String,
  mut req: ServiceRequest,
  next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
  // pre-processing
  let headers = req.headers_mut();
  headers.insert("x-user-name".parse().unwrap(), "AJ Pillai".parse().unwrap());

  if !body.is_empty() {
    let mut json_v: Value = serde_json::from_str::<Value>(&body).unwrap();

    // Check if the parsed value is an object and get a mutable reference
    if let Some(obj) = json_v.as_object_mut() {
      // Add a new key-value pair to the JSON object
      obj.insert("email".to_string(), json!("john.doe@example.com"));
    }

    let new_body = serde_json::to_vec(&json_v)?;
    let (_, mut payload) = Payload::create(true);
    payload.unread_data(new_body.into());
    req.set_payload(payload.into());
  }

  println!("my_mw: {}", body.is_empty());

  next.call(req).await
}
