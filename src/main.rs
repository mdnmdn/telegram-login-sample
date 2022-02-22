use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use actix_files::Files;
use env_logger::Env;

#[actix_web::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: i32 = std::env::var("PORT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(4050);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("starting...");

    HttpServer::new(move || {
        App::new()
          .wrap(Logger::default())
          .service(Files::new("/", "/static").index_file("index.html"))
          .route("/wow", web::to(|| HttpResponse::Ok().body("wow!")))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .workers(2)
    .run()
    .await?;

    Ok(())
}
