use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebResponse<T> {
    pub code: u32,
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}
