use std::{str::from_utf8, sync::Arc};

use anyhow::{Context, Ok, Result};
use serde::Deserialize;
use serde_json::{from_value, json, Value};
use warp::hyper::Response;
use warp::{hyper::body::Bytes, Filter};

use crate::model::{init_db, UserMac, UserPatch};
use crate::web::handle_rejection;

use super::user_rest_filters;

#[tokio::test]
async fn web_user_register() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let user_apis = user_rest_filters(db.clone()).recover(handle_rejection);

    let body = json!({
      "username": "John",
      "password": "123"
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("POST")
        .path("/user/register")
        .json(&body)
        .reply(&user_apis)
        .await;

    // CHECK
    assert_eq!(200, resp.status());

    Ok(())
}

#[tokio::test]
async fn web_user_login() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let user_apis = user_rest_filters(db.clone()).recover(handle_rejection);

    let data_fx = UserPatch {
        username: String::from("john"),
        password: String::from("123"),
    };

    let body = json!({
      "username": "john",
      "password": "123"
    });

    // -- ACTION
    // registering user
    let user = UserMac::register(&db, data_fx.clone()).await?;

    let resp = warp::test::request()
        .method("POST")
        .path("/user/login")
        .json(&body)
        .reply(&user_apis)
        .await;

    // CHECK
    assert_eq!(200, resp.status());

    let resp_data: i64 = extract_body_data(resp)?;

    assert_eq!(user.id, resp_data);

    Ok(())
}

fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where
    for<'de> D: Deserialize<'de>,
{
    // parse the body as serde_json::Value
    let body = from_utf8(resp.body())?;
    let mut body: Value = serde_json::from_str(body)
        .with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", body))?;

    // extract data
    let data = body["data"].take();

    // deserialize the data to D
    let data: D = from_value(data)?;

    Ok(data)
}
