use std::sync::mpsc;

use models::Person;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum DbAction {
    CreatePerson(Person),
    GetPerson(String),
    GetAllPersons,
}

#[derive(Debug)]
pub struct DbRequest {
    pub action: DbAction,
    pub responder: mpsc::Sender<DbResponse>,
}

#[derive(Debug)]
pub enum DbResponse {
    Success(Vec<Person>),
    Err(String),
}
pub struct DbClient {
    pub db: Surreal<Client>,
    pub receiver: Mutex<mpsc::Receiver<DbRequest>>,
}

impl DbClient {
    pub fn new(db: Surreal<Client>, receiver: Mutex<mpsc::Receiver<DbRequest>>) -> Self {
        Self { db, receiver }
    }

    pub async fn create_db() -> Surreal<Client> {
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
