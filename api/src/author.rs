use ormx::Insert;
use rocket::{Route, form::{ FromForm, Form }, http::Status, response::status::Custom, serde::json::{Json}};
use sqlx::PgPool;
use crate::{models::{Author, InsertAuthor}, tools::{Res, acquire_db}};


#[rocket::get("/")]
pub async fn list_authors(pool: &rocket::State<PgPool>, ) -> Res<Vec<Author>> {
    let mut db = acquire_db(pool).await?;

    ormx::conditional_query_as!(
        Author,
        "SELECT * FROM authors ORDER BY name;"
    )
        .fetch_all(&mut *db)
        .await
        .map(Json)
        .map_err(|_| Custom(Status::InternalServerError, String::from("Failed loading authors.")))
}

#[derive(FromForm)]
pub struct CreateAuthor {
    name: String,
}

impl From<InsertAuthor> for CreateAuthor {
    fn from(model: InsertAuthor) -> Self {
        Self {
            name: model.name,
        }
    }
}

#[rocket::post("/", data = "<input>")]
pub async fn create_author(pool: &rocket::State<PgPool>, input: Form<CreateAuthor>) -> Res<Author> {
    let mut db = acquire_db(pool).await?;

    InsertAuthor {
        name: input.name.clone()
    }
        .insert(&mut *db)
        .await
        .map(Json)
        .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

pub fn author_routes() -> Vec<Route> {
  rocket::routes![
      list_authors,
      create_author
  ]
}
