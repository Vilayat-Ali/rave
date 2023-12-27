pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub sender_id: String,
    pub receiver_id: String,
    pub gang_id: Option<String>,
    pub message: String,
}

impl Chat {
    pub fn new(
        sender_id: String,
        receiver_id: String,
        gang_id: Option<String>,
        message: String,
    ) -> Self {
        Self {
            sender_id,
            receiver_id,
            gang_id,
            message,
        }
    }
}
