use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use super::db_config::DbConfig;

#[derive(Clone)]
pub struct DbClient {
    pub db: Surreal<Client>,
}

impl DbClient {
    pub async fn create(db_config: DbConfig) -> Surreal<Client> {
        let db = DbClient::create_db(db_config).await;
        db
    }

    async fn create_db(
        DbConfig {
            db_url,
            db_username,
            db_password,
            db_ns,
            db_name,
        }: DbConfig,
    ) -> Surreal<Client> {
        let db = Surreal::new::<Ws>(db_url).await.unwrap();
        db.signin(Root {
            username: db_username,
            password: db_password,
        })
        .await
        .unwrap();
        db.use_ns(db_ns).use_db(db_name).await.unwrap();
        db
    }
}
