use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
  HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
  email: String,
  name: String
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(||
      App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscription", web::post().to(subscribe))
    )
    .listen(listener)?
    .run();

  Ok(server)
}
