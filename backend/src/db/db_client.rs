use models::Person;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc,
};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::sync::Mutex;

use crate::db::{DbAction, DbResponse};

use super::{db_config::DbConfig, DbRequest};
pub struct DbClient {
    pub db: Surreal<Client>,
    pub receiver: Mutex<Receiver<DbRequest>>,
}

impl DbClient {
    pub async fn create(db_config: DbConfig) -> (Mutex<Self>, Arc<Mutex<Sender<DbRequest>>>) {
        let (req_send, req_recv) = channel::<DbRequest>();
        let db = DbClient::create_db(db_config).await;
        let sender = Arc::new(Mutex::new(req_send));
        let receiver = Mutex::new(req_recv);
        (Mutex::new(Self { db, receiver }), Arc::clone(&sender))
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

    pub async fn listen(&self) {
        let receiver = self.receiver.lock().await;
        loop {
            let receive = receiver.recv();
            println!("Received request!");
            match receive {
                Ok(DbRequest { action, responder }) => {
                    let query = match action {
                        DbAction::GetAllPersons => "SELECT * FROM persons",
                        _ => "SELECT * FROM persons",
                    };
                    println!("Query DB");
                    let response = self.db.query(query).await;
                    let response: Vec<Person> = match response {
                        Ok(mut response) => {
                            println!("OK response");
                            let response: Vec<Person> = match response.take(0) {
                                Ok(result) => {
                                    println!("Ok result - vec found");
                                    result
                                }
                                Err(err) => {
                                    dbg!(err);
                                    vec![]
                                }
                            };
                            response
                        }
                        Err(err) => {
                            dbg!(&err);
                            vec![]
                        }
                    };
                    let send = responder.send(DbResponse::Success(response));
                    match send {
                        Ok(()) => println!("Response Sent!"),
                        Err(err) => {
                            println!("Failed to send response!");
                            dbg!(err);
                            ()
                        }
                    }
                }
                Err(err) => {
                    dbg!(err);
                    ()
                }
            }
        }
    }
}
