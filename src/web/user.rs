use std::sync::Arc;

use warp::reply::Json;
use warp::Filter;

use crate::model::{Db, UserMac, UserPatch};

pub fn user_rest_filters(
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let user_path = warp::path("user");
    let common = super::filter_utils::with_db(db.clone());

    // REGISTER `POST /user/register`
    let register_path = user_path.and(warp::path("register"));
    let register = register_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(user_register);

    // LOGIN `POST /user/login`
    let login_path = user_path.and(warp::path("login"));
    let login = login_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(user_login);

    register.or(login)
}

async fn user_login(db: Arc<Db>, data: UserPatch) -> Result<Json, warp::Rejection> {
    let user = UserMac::login(&db, data).await?;
    super::json_response(user.id)
}

async fn user_register(db: Arc<Db>, data: UserPatch) -> Result<Json, warp::Rejection> {
    let user = UserMac::register(&db, data).await?;
    super::json_response(user.id)
}

#[cfg(test)]
#[path = "../_tests/web_user.rs"]
mod tests;
