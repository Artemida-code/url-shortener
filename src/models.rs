use std::collections::HashMap;
use std::sync::Mutex;
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

pub struct DataLinks {
    pub links: Mutex<HashMap<String, String>>,
}