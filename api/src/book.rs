use ormx::Insert;
use rocket::{Route, FromForm, http::Status, response::status::Custom, serde::{json::{Json}, Deserialize}};
use sqlx::PgPool;
use crate::{PAGE_SIZE, models::{Book, InsertBook}, tools::{Res, acquire_db}};

#[derive(FromForm)]
pub struct BookListFilters {
  author_id: Option<u32>,
}

#[rocket::get("/?<page>&<filters..>")]
async fn list_books(pool: &rocket::State<PgPool>, filters: BookListFilters, page: Option<u32>) -> Res<Vec<Book>> {
    let mut db = acquire_db(pool).await?;

    let skip: u32 = match page {
      None | Some(0) => 0,
      Some(page) => (page - 1) * PAGE_SIZE
    };

    ormx::conditional_query_as!(
        Book,
        "SELECT * FROM books"
        "WHERE 1=1"
        Some(id) = filters.author_id => {
          "AND author_id="?(id as i64)
        }
        "ORDER BY title"
        "LIMIT" ?(PAGE_SIZE as i64)
        "OFFSET" ?(skip as i64)
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