mod handlers;
mod models;
use crate::{handlers::*, models::*};
use actix_web::{App, HttpServer, web, web::Data};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_db();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/url", web::post().to(shorten_url))
            .route("/{alias}", web::get().to(redirect_url))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}







