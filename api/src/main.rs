extern crate dotenv;

extern crate rocket;
use crate::models::{Book,InsertBook};
use dotenv::dotenv;
use ormx::Insert;
use rocket::Error;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::response::{Responder, status::{Custom}};
use rocket::serde::json::{Json};
use sqlx::postgres::PgPool;
mod models;

//mod models;

// #[rocket::get("/book")]
// fn index() -> Json<Vec<models::Book>> {

// }

#[derive(FromForm)]
struct CreateBook {
    author_id: i32,
    title: String,
    genre: models::Genre
}

impl From<models::InsertBook> for CreateBook {
    fn from(model: models::InsertBook) -> Self {
        Self {
            author_id: model.author_id,
            title: model.title,
            genre: model.genre,
        }
    }
}

#[rocket::post("/book", data = "<input>")]
async fn createBook(pool: &rocket::State<PgPool>, input: Form<CreateBook>) -> Result<Json<Book>, Custom<String>> {
    let mut db = pool.acquire()
        .await
        .map_err(|e| Custom(Status::InternalServerError, String::from("Error acquiring pool")))?;

    let book = InsertBook {
        author_id: input.author_id,
        genre: input.genre,
        title: input.title.clone()
    }
        .insert(&mut *db)
        .await
        .map_err(|e| Custom(Status::InternalServerError, String::from("Error inserting")))?;

    Ok(Json(book))
}

#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    rocket::build()
        .manage::<PgPool>(pool)
        .mount("/", rocket::routes![createBook])
}
