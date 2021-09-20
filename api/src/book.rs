use ormx::Insert;
use rocket::{Route, form::{ FromForm, Form }, http::Status, response::status::Custom, serde::json::{Json}};
use sqlx::PgPool;
use crate::{models::{Book, InsertBook}, tools::{Res, acquire_db}};


#[rocket::get("/")]
async fn list_books(pool: &rocket::State<PgPool>, ) -> Res<Vec<Book>> {
    let mut db = acquire_db(pool).await?;

    ormx::conditional_query_as!(
        Book,
        "SELECT * FROM books ORDER BY title;"
    )
        .fetch_all(&mut *db)
        .await
        .map(Json)
        .map_err(|_| Custom(Status::InternalServerError, String::from("Failed loading books.")))
}

#[derive(FromForm)]
pub struct CreateBook {
    author_id: i32,
    title: String,
    genre: i16
}

impl From<InsertBook> for CreateBook {
    fn from(model: InsertBook) -> Self {
        Self {
            author_id: model.author_id,
            title: model.title,
            genre: model.genre,
        }
    }
}

#[rocket::post("/", data = "<input>")]
async fn create_book(pool: &rocket::State<PgPool>, input: Form<CreateBook>) -> Res<Book> {
    let mut db = acquire_db(pool).await?;

    InsertBook {
        author_id: input.author_id,
        genre: input.genre,
        title: input.title.clone()
    }
        .insert(&mut *db)
        .await
        .map(Json)
        .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

pub fn book_routes () -> Vec<Route> {
    rocket::routes![
        list_books,
        create_book
    ]
}