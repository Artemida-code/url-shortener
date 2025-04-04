use rusqlite::{params, Connection};
use actix_web::{web, HttpResponse};
use rand::Rng;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use crate::{Link, ReturnLink, Output};

pub fn init_db() -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::file("db1.db");
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let conn = pool.get().expect("Failed to get a connection from the pool.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS url (
            id INTEGER PRIMARY KEY,
            alias TEXT NOT NULL UNIQUE,
            long_url TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create table.");

    pool
}

pub async fn alias_creation() -> Result<String, rusqlite::Error> {
    let mut rng = rand::rng();
    let mut alias = String::new();
    for _ in 1..11 {
        let ch = rng.random_range(128512..=128591) as u32;
        alias.push(char::from_u32(ch).unwrap())
    }
    Ok(alias)
}


fn save_url(conn: &Connection, alias: &str, long_url: &str) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO url (alias, long_url) VALUES (?1, ?2)",
        params![alias, long_url],
    )?;
    Ok(())
}

fn get_url(conn: &Connection, alias: &str) -> Result<String, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT long_url FROM url WHERE alias = ?1")?;
    let long_url: String = stmt.query_row(params![alias], |row| row.get(0))?;
    Ok(long_url.to_string())
}

pub async fn shorten_url(
    data: web::Json<Link>,
    pool: web::Data<Pool<SqliteConnectionManager>>
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    if data.alias.as_ref().map_or(true, |alias| alias.is_empty()) {
        let alias = alias_creation().await.unwrap();
        match save_url(&conn, &alias, &data.url) {
            Ok(_) => HttpResponse::Created().json(ReturnLink {
                status: "ok".to_string(),
                error: None,
                alias: Some(alias),
            }),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    } else {
        let alias = data.alias.as_ref().unwrap().clone();
        match save_url(&conn, &alias, &data.url) {
            Ok(_) => HttpResponse::Created().json(ReturnLink {
                status: "ok".to_string(),
                error: None,
                alias: Some(alias),
            }),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

pub async fn redirect_url(
    alias: web::Path<String>,
    pool: web::Data<Pool<SqliteConnectionManager>>
) -> HttpResponse {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    match get_url(&conn, &alias) {
        Ok(long_url) => HttpResponse::Created().json(Output{
            status: long_url.clone(),
        }),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
