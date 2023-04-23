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

use super::DbRequest;
pub struct DbClient {
    pub db: Surreal<Client>,
    pub receiver: Mutex<Receiver<DbRequest>>,
}

impl DbClient {
    pub async fn create() -> (Mutex<Self>, Arc<Mutex<Sender<DbRequest>>>) {
        let (req_send, req_recv) = channel::<DbRequest>();
        let db = DbClient::create_db().await;
        let sender = Arc::new(Mutex::new(req_send));
        let receiver = Mutex::new(req_recv);
        (Mutex::new(Self { db, receiver }), Arc::clone(&sender))
    }

    async fn create_db() -> Surreal<Client> {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
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
