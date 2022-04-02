use std::sync::Arc;

use model::init_db;
use web::start_web;

mod model;
mod security;
mod web;

const DEFAULT_WEB_PORT: u16 = 5000;

#[tokio::main]
async fn main() {
    let web_port = DEFAULT_WEB_PORT;

    // get the database
    // TODO: loop until valid DB
    let db = init_db().await.expect("Cannot init db");
    let db = Arc::new(db);

    // start the server
    match start_web(web_port, db).await {
        Ok(_) => println!("Server ended"),
        Err(ex) => println!("ERROR - web server failed to start. Cause {:?}", ex),
    }
}
