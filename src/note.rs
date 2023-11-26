use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Note {
    id: usize,
    title: String,
    content: String,
    created_at: DateTime<Utc>
}