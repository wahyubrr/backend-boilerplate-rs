use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub access_status: i8,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}
