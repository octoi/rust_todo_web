use std::sync::Arc;

use warp::Filter;

use crate::{
    model::Db,
    security::{utx_from_token, UserCtx},
};

pub fn do_auth(_db: Arc<Db>) -> impl Filter<Extract = (UserCtx,), Error = warp::Rejection> + Clone {
    warp::any().and_then(|| async { Ok::<UserCtx, warp::Rejection>(utx_from_token("123").await?) })
}
