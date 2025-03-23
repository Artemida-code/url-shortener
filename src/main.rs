mod handlers;
mod models;
use crate::{handlers::*, models::*};
use actix_web::{App, HttpServer, Responder, web};
use std::{collections::HashMap, sync::Mutex};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let links = web::Data::new(DataLinks {
        links: Mutex::new(HashMap::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(links.clone())
            .route("/url", web::post().to(link_creation))
            .route("/{url}", web::get().to(get_link))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
