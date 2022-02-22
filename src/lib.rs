use actix_web::{HttpRequest, Responder, HttpResponse, post};


#[post("/bot_hook_221")]
pub async fn bot_web_hook(_req: HttpRequest) -> impl Responder {
  HttpResponse::Ok()
}
