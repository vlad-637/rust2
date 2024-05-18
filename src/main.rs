use std::{collections::HashMap, sync::{Arc, RwLock}};
use serde::Serialize;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: Uuid,
    text: String, 
    complete: bool,
}