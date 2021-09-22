use ormx::{Table, Patch};
use rocket::form::{FromForm};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Table, FromForm, Serialize, Deserialize)]
#[ormx(table = "books", id = id, insertable)]
pub struct Book {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub genre: i16
}

#[derive(Debug, Table, FromForm, Serialize, Deserialize)]
#[ormx(table = "authors", id = id, insertable)]
pub struct Author {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub name: String
}

#[derive(Patch)]
#[ormx(table_name = "authors", table = crate::models::Author, id = "id")]
pub struct UpdateAuthor {
    pub name: String
}