use actix_web::{post, web, HttpResponse, Responder};

#[post("/bot_hook_221")]
pub async fn bot_web_hook(bytes: web::Bytes) -> impl Responder {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => {
            if !text.is_empty() {
                log::info!("body: {}\n", text);
            }
        }
        Err(_) => log::warn!("Error parsing body"),
    };

    HttpResponse::Ok()
}
