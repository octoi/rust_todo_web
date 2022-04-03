use serde::{Deserialize, Serialize};
use sqlb::{HasFields, Raw};

use super::db::Db;
use crate::{model, security::UserCtx};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, // creator id
    pub title: String,
    pub status: TodoStatus,
}

#[derive(sqlb::Fields, Debug, Default, Clone, Deserialize)]
pub struct TodoPatch {
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}

sqlb::bindable!(TodoStatus);

pub struct TodoMac;

impl TodoMac {
    const TABLE: &'static str = "todo";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];
}

impl TodoMac {
    pub async fn create(db: &Db, utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        let mut fields = data.fields();
        fields.push(("cid", utx.user_id).into());
        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn get(db: &Db, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let result = sb.fetch_one(db).await;

        model::handle_fetch_one_result(result, Self::TABLE, id.to_string())
    }

    pub async fn update(
        db: &Db,
        utx: &UserCtx,
        id: i64,
        data: TodoPatch,
    ) -> Result<Todo, model::Error> {
        let mut fields = data.fields();
        // augment the fields with the cid/ctime
        fields.push(("mid", utx.user_id).into());
        fields.push(("ctime", Raw("now()")).into());

        let sb = sqlb::update()
            .table(Self::TABLE)
            .data(fields)
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let result = sb.fetch_one(db).await;

        model::handle_fetch_one_result(result, Self::TABLE, id.to_string())
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("!id");

        // execute the query
        let todos = sb.fetch_all(db).await?;

        Ok(todos)
    }

    pub async fn delete(db: &Db, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
        let sb = sqlb::delete()
            .table(Self::TABLE)
            .returning(Self::COLUMNS)
            .and_where_eq("id", id);

        let result = sb.fetch_one(db).await;

        model::handle_fetch_one_result(result, Self::TABLE, id.to_string())
    }
}

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
