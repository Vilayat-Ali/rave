use mongodb::{bson::Document, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserName {
    pub first: String,
    pub last: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: UserName,
    pub email: String,
    pub wallet_address: String,
    pub nationality: String,
    pub bio: Option<String>,
}

pub struct UserModel {
    collection: mongodb::Collection<User>,
}

impl UserModel {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("user"),
        }
    }

    // create
    pub async fn create(
        &mut self,
        first_name: String,
        last_name: String,
        email: String,
        wallet_address: String,
        nationality: String,
        bio: Option<String>,
    ) {
        self.collection
            .insert_one(
                User {
                    name: UserName {
                        first: first_name,
                        last: last_name,
                    },
                    email,
                    wallet_address,
                    nationality,
                    bio,
                },
                None,
            )
            .await
            .unwrap();
    }
}
