use std::sync::Arc;
use warp::{reply::Json, Filter};

use crate::{
    model::{Db, TodoMac, TodoPatch},
    security::UserCtx,
};

use super::filter_auth::do_auth;

pub fn todo_rest_filters(
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_path = warp::path("todos");
    let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));

    // LIST todos `GET todos/`
    let list = todos_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(todo_list);

    // GET todo `GET /todos/100`
    let get = todos_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(todo_get);

    // CREATE todo `POST /todos with body TodoPatch`
    let create = todos_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(todo_create);

    // UPDATE todo `PATCH /todos/100 with body TodoPatch`
    let update = todos_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(todo_update);

    // DELETE todo `DELETE /todos/100`
    let delete = todos_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(todo_delete);

    list.or(get).or(create).or(update).or(delete)
}

async fn todo_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let todos = TodoMac::list(&db, &utx).await?;
    super::json_response(todos)
}

async fn todo_get(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let todo = TodoMac::get(&db, &utx, id).await?;
    super::json_response(todo)
}

async fn todo_create(db: Arc<Db>, utx: UserCtx, patch: TodoPatch) -> Result<Json, warp::Rejection> {
    let todo = TodoMac::create(&db, &utx, patch).await?;
    super::json_response(todo)
}

async fn todo_update(
    db: Arc<Db>,
    utx: UserCtx,
    id: i64,
    patch: TodoPatch,
) -> Result<Json, warp::Rejection> {
    let todo = TodoMac::update(&db, &utx, id, patch).await?;
    super::json_response(todo)
}

async fn todo_delete(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let todo = TodoMac::delete(&db, &utx, id).await?;
    super::json_response(todo)
}

#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod tests;
