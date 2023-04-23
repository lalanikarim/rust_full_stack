use std::sync::{mpsc, Arc};

use tokio::sync::Mutex;

use crate::db::DbRequest;
#[derive(Clone)]
pub struct AppState {
    pub req_send: Arc<Mutex<mpsc::Sender<DbRequest>>>,
}

impl AppState {
    pub fn new(req_send: Arc<Mutex<mpsc::Sender<DbRequest>>>) -> Self {
        Self { req_send }
    }
}
