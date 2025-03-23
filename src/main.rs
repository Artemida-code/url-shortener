use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

#[derive(Deserialize)]
pub struct Link {
    pub url: String,
    pub alias: Option<String>,
}

#[derive(Serialize)]
pub struct ReturnLink {
    pub status: String,
    pub error: Option<actix_web::Error>,
    pub alias: Option<String>,
}

pub struct DataLinks {
    pub links: Mutex<HashMap<String, String>>,
}

async fn link_creation(json: web::Json<Link>, data_links: web::Data<DataLinks>) -> impl Responder {
    let mut data = data_links.links.lock().unwrap();
    if let Some(_) = json.alias {
        data.insert(json.url.clone(), json.alias.clone().unwrap());
    } else {
        let mut rng = rand::rng();
        let mut vec_alias = Vec::new();
        for _ in 1..11 {
            let ch = rng.random_range(128512..=128591) as u32;
            vec_alias.push(char::from_u32(ch).unwrap());
        }
        data.insert(json.url.clone(), vec_alias.into_iter().collect());
    }
    HttpResponse::Ok().json(ReturnLink {
        status: "ok".to_string(),
        error: None,
        alias: data.get(&json.url).cloned(),
    })
}

async fn get_link(path: web::Path<String>, links: web::Data<DataLinks>) -> impl Responder {
    let url = path.into_inner();
    if let Some(url) = links.links.lock().unwrap().get(&url) {
        HttpResponse::Ok().body(format!("Alias: {}", url))
    } else {
        HttpResponse::NotFound().body("404: Not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut links = web::Data::new(DataLinks {
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
