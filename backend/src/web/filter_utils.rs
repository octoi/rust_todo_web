use std::{convert::Infallible, sync::Arc};

use warp::Filter;

use crate::model::Db;

pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
