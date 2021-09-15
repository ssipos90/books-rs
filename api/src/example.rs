use std::io::{Error};
use sqlx::{Type};
use ormx::{Table};
use serde::{Deserialize};
use rocket::form::{FromForm, FromFormField};

#[derive(Debug, Table)]
#[ormx(table = "books", id = id, insertable)] // insertable generates struct InsertBook
pub struct Book {
    #[ormx(column = "id", default)]
    pub id: i32,
    pub title: String,
    // ... other fields
}

#[tokio::main]
async fn main () -> Result<(), Error> {
    // init code
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await?;
    let book = InsertBook {
      title
    }
      .insert(&mut *pool.acquire().await?)
      .await?;
}
