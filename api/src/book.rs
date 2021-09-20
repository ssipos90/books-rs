use ormx::Insert;
use rocket::{Route, http::Status, response::status::Custom, serde::{json::{Json}, Deserialize}};
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

#[derive(Deserialize)]
enum Genre {
  SF,
  Fiction,
  Drama
}

#[derive(Deserialize)]
pub struct CreateBook<'r> {
    author_id: i32,
    title: &'r str,
    genre: Genre
}

#[rocket::post("/", format = "application/json", data = "<input>")]
async fn create_book<'r>(pool: &rocket::State<PgPool>, input: Json<CreateBook<'r>>) -> Res<Book> {
    let mut db = acquire_db(pool).await?;

    InsertBook {
        author_id: input.author_id,
        genre: match input.genre {
          Genre::Drama => 1,
          Genre::SF => 2,
          Genre::Fiction => 3,
        },
        title: input.title.to_string()
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