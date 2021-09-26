use crate::{
    models::{Book, InsertBook},
    tools::{acquire_db, Res},
    PAGE_SIZE,
};
use ormx::{Insert, Table};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::{json::Json, Deserialize},
    FromForm, Route,
};
use sqlx::PgPool;

#[derive(FromForm)]
struct BookListFilters<'r> {
    author_id: Option<u32>,
    title: Option<&'r str>,
}

#[rocket::get("/?<page>&<filters..>")]
async fn list_books<'r>(
    pool: &rocket::State<PgPool>,
    filters: BookListFilters<'r>,
    page: Option<u32>,
) -> Res<Vec<Book>> {
    let mut db = acquire_db(pool).await?;

    let skip: u32 = match page {
        None | Some(0) => 0,
        Some(page) => (page - 1) * PAGE_SIZE,
    };

    ormx::conditional_query_as!(
        Book,
        "SELECT id, author_id, isbn, title, genre"
        "FROM books"
        "WHERE 1=1"
        Some(id) = filters.author_id => {
          "AND author_id="?(id as i64)
        }
        Some(title) = filters.title => {
          "AND title LIKE "?(format!("%{}%", title))
        }
        "ORDER BY title"
        "LIMIT" ?(PAGE_SIZE as i64)
        "OFFSET" ?(skip as i64)
    )
    .fetch_all(&mut *db)
    .await
    .map(Json)
    .map_err(|_| {
        Custom(
            Status::InternalServerError,
            String::from("Failed loading books."),
        )
    })
}

#[derive(Clone, Copy, Deserialize)]
enum Genre {
    Drama = 1,
    SF = 2,
    Fiction = 3,
}

#[derive(Deserialize)]
pub struct CreateBook<'r> {
    author_id: i32,
    isbn: &'r str,
    title: &'r str,
    genre: Genre,
}

#[rocket::post("/", format = "application/json", data = "<input>")]
async fn create_book<'r>(pool: &rocket::State<PgPool>, input: Json<CreateBook<'r>>) -> Res<Book> {
    let mut db = acquire_db(pool).await?;

    InsertBook {
        author_id: input.author_id,
        genre: input.genre as i16,
        isbn: input.isbn.to_string(),
        title: input.title.to_string(),
    }
    .insert(&mut *db)
    .await
    .map(Json)
    .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

#[derive(Deserialize)]
struct UpdateBookBody<'r> {
    title: &'r str,
    isbn: &'r str,
}

#[rocket::put("/<book_id>", format = "application/json", data = "<input>")]
async fn update_book<'r>(
    pool: &rocket::State<PgPool>,
    book_id: i32,
    input: Json<UpdateBookBody<'r>>,
) -> Res<Book> {
    let mut db = acquire_db(pool).await?;

    let mut book = Book::get(&mut *db, book_id).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            Custom(Status::NotFound, format!("book_id {} not found", book_id))
        }
        _ => Custom(
            Status::InternalServerError,
            String::from("Error fetching from database."),
        ),
    })?;

    validate_isbn(input.isbn)
        .map_err(|_| Custom(Status::UnprocessableEntity, String::from("Error inserting")))?;

    book.title = input.title.to_string();
    book.isbn = input.isbn.to_string();
    book.update(&mut *db)
        .await
        .map(|_| Json(book))
        .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

pub enum ErrorISBN {
    InvalidLength(usize),
    InvalidChars,
    ControlMismatch,
}

pub fn validate_isbn<'r>(input: &'r str) -> Result<(), ErrorISBN> {
  if input.len() != 13 {
    return Err(ErrorISBN::InvalidLength(input.len()));
  }

  let mut some_digits = input.chars()
    .map(|c| c.to_digit(10));

  if some_digits.any(|x| x.is_none()) {
    return Err(ErrorISBN::InvalidChars);
  }
  let mut digits = some_digits.map(|x| x.unwrap());

  let control = digits.nth(13).unwrap();

  let sum = digits
    .take(12)
    .enumerate()
    .map(|(i, d)| {
      if i & 1 == 0 {
        d
      } else {
        d * 3
      }
    })
    .sum::<u32>();

  if sum == 0 {
    return match control {
      0 => Ok(()),
      _ => Err(ErrorISBN::ControlMismatch)
    };
  } else if 10 - sum != control {
      return Err(ErrorISBN::ControlMismatch);
  }

  Ok(())
}

pub fn book_routes() -> Vec<Route> {
    rocket::routes![list_books, create_book, update_book]
}
