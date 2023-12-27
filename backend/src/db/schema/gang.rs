pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Gang {
    pub name: String,
    pub leader_id: String,
    pub members: Vec<String>,
    pub description: Option<String>,
    pub level: u16,
    pub country_of_origin: String,
}

impl Gang {
    pub fn new(
        name: String,
        leader_id: String,
        members: Vec<String>,
        description: Option<String>,
        level: u16,
        country_of_origin: String,
    ) -> Self {
        Self {
            name,
            leader_id,
            members,
            description,
            level,
            country_of_origin,
        }
    }
}
