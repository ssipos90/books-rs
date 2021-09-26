use ormx::Table;
use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Table, FromForm, Serialize, Deserialize)]
#[ormx(table = "books", id = id, insertable)]
pub struct Book {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub author_id: i32,
    pub isbn: String,
    pub title: String,
    pub genre: i16,
}

#[derive(Debug, Table, FromForm, Serialize, Deserialize)]
#[ormx(table = "authors", id = id, insertable)]
pub struct Author {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub name: String,
}
