use sqlx::{Type};
use ormx::{Table};
use serde::{Deserialize};
use rocket::form::{FromForm, FromFormField};

#[derive(Debug, Copy, Clone, Deserialize, FromFormField, Type)]
#[sqlx(type_name = "genre")]
#[sqlx(rename_all = "lowercase")] 
pub enum Genre {
    SF,
    Fiction,
    Psychology,
    Other
}

#[derive(Debug, Table, FromForm)]
#[ormx(table = "books", id = id, insertable)]
pub struct Book {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    #[ormx(custom_type)]
    pub genre: Genre
}

#[derive(Debug, Table)]
#[ormx(table = "authors", id = id, insertable)]
pub struct Author {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub name: String,
    #[ormx(custom_type)]
    pub genre: Option<Genre>
}
