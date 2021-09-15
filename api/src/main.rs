extern crate dotenv;

use dotenv::dotenv;
use ormx::Insert;
use rocket::form::{Form, FromForm};
use rocket::response::status::NotFound;
use rocket::{Build, Error, Rocket, data::FromData, response::content::Json};
use rocket::serde::{Deserialize};
use sqlx::postgres::PgPool;
extern crate rocket;

mod models;

//mod models;

// #[rocket::get("/book")]
// fn index() -> Json<Vec<models::Book>> {

// }

struct CreateBook {
    author_id: u32;
}

impl From<InsertBook> for FromFormStruct {

}

#[rocket::post("/book", data = "<input>")]
async fn world(pool: &rocket::State<PgPool>, input: Form<models::InsertBook>) -> Result<Json<models::Book>, Error> {
    match pool.acquire().await {
        Ok(db) => {
            let book = models::InsertBook {
                author_id: input.author_id,
                genre: input.genre,
                title: input.title
            }.insert(&mut *db).await;

            Ok(book)
        },
        Err(e) => Err(NotFound)
    }
}

#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();
    let pool = PgPool::connect(&dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    rocket::build()
        .manage::<PgPool>(pool)
        .mount("/", rocket::routes![world])
}
