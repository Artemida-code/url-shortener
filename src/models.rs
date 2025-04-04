use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Link {
    pub url: String,
    pub alias: Option<String>,
}

#[derive(Serialize)]
pub struct ReturnLink {
    pub status: String,
    pub error: Option<String>,
    pub alias: Option<String>,
}

#[derive(Serialize)]
pub struct Output {
    pub status: String,
}

