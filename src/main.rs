use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger };
use actix_files::Files;
use env_logger::Env;
use sample_telegram_login::bot_web_hook;

#[actix_web::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let port: i32 = std::env::var("PORT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(4050);

    let binding = format!("0.0.0.0:{port}");

    log::info!("starting on {binding}...");

    HttpServer::new(move || {
        App::new()
          .wrap(Logger::default())
          .service(bot_web_hook)
          .route("/wow", web::get().to(move || async { HttpResponse::Ok().body("wow!") }))
          .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(binding)?
    .workers(2)
    .run()
    .await?;

    Ok(())
}
