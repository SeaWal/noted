use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// TODO: make this configurable from app.rs
pub const DB_PATH: &str = "./notes/notes.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    id: usize,
    title: String,
    content: String,
    created_at: DateTime<Utc>
}