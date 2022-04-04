use crate::{
    model::{self, Db},
    security,
    web::{todo::todo_rest_filters, user::user_rest_filters},
};
use serde::Serialize;
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{reply::Json, Filter, Rejection, Reply};

mod filter_auth;
mod filter_utils;
mod todo;
mod user;

pub async fn start_web(web_port: u16, db: Arc<Db>) -> Result<(), Error> {
    // Apis
    let todo_api = todo_rest_filters(db.clone());
    let user_api = user_rest_filters(db);
    let routes = todo_api.or(user_api).recover(handle_rejection);

    println!("[+] RUNNING WEB SERVER ON 127.0.0.1:{}", web_port);
    println!("🚀 http://127.0.0.1:{}", web_port);
    println!("🚀 http://localhost:{}", web_port);
    warp::serve(routes).run(([127, 0, 0, 1], web_port)).await;

    Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    // Print to server side
    println!("ERROR - {:?}", err);

    // TODO: Call log API for capture and store

    // Build user message
    let user_message = match err.find::<WebErrorMessage>() {
        Some(err) => err.message.to_string(),
        None => "Unknown".to_string(),
    };

    let result = json!({ "errorMessage": user_message });
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(
        result,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}

pub fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data });
    Ok(warp::reply::json(&response))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String),

    #[error("Fail authentication missing X-Auth-Token header.")]
    FailAuthMissingXAuth,
}

// Warp custom error
#[derive(Debug)]
pub struct WebErrorMessage {
    pub typ: &'static str,
    pub message: String,
}

impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage { typ, message })
    }
}

impl From<self::Error> for warp::Rejection {
    fn from(other: self::Error) -> Self {
        WebErrorMessage::rejection("web::Error", format!("{}", other))
    }
}

impl From<model::Error> for warp::Rejection {
    fn from(other: model::Error) -> Self {
        WebErrorMessage::rejection("model::Error", format!("{}", other))
    }
}

impl From<security::Error> for warp::Rejection {
    fn from(other: security::Error) -> Self {
        WebErrorMessage::rejection("security::Error", format!("{}", other))
    }
}
