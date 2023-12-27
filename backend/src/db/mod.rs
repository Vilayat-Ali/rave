pub mod schema;

use crate::db::schema::user::UserModel;
use mongodb::{options::ClientOptions, Client, Database};

pub struct Mongo;

impl Mongo {
    pub async fn establish_conn() -> Database {
        let client_options = ClientOptions::parse("mongodb://127.0.0.1:27017/rave")
            .await
            .unwrap();
        let client = Client::with_options(client_options).unwrap();

        let db = client.default_database().unwrap();
        for collection_name in db.list_collection_names(None).await.unwrap() {
            println!("{}", collection_name);
        }
        UserModel::new(&db)
            .create(
                "Vilayat".to_owned(),
                "Ali".to_owned(),
                "Vilayatcodemysite@gmail.com".to_owned(),
                "0x343rew4544y64t455".to_owned(),
                "IND".to_owned(),
                None,
            )
            .await;
        db
    }
}
