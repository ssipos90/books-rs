extern crate dotenv;
extern crate rocket;
mod models;
mod author;
mod book;
mod tools;
use crate::author::{author_routes};
use crate::book::{book_routes};
use dotenv::dotenv;
use sqlx::postgres::PgPool;

#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    rocket::build()
        .manage::<PgPool>(pool)
        .mount("/author", author_routes())
        .mount("/book", book_routes())
}
