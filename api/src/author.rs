use ormx::Insert;
use rocket::{Route, FromForm, http::Status, response::status::Custom, serde::{json::{Json}, Deserialize}};
use sqlx::PgPool;
use crate::{PAGE_SIZE, models::{Author, InsertAuthor}, tools::{Res, acquire_db}};

#[derive(FromForm)]
pub struct AuthorFilters<'r> {
  name: Option<&'r str>
}

#[rocket::get("/?<page>&<filters..>")]
pub async fn list_authors<'r>(pool: &rocket::State<PgPool>, filters: AuthorFilters<'r>, page: Option<u32>) -> Res<Vec<Author>> {
    let mut db = acquire_db(pool).await?;

    let skip: u32 = match page {
      None | Some(0) => 0,
      Some(page) => (page - 1) * PAGE_SIZE
    };

    ormx::conditional_query_as!(
        Author,
        "SELECT * FROM authors"
        "WHERE 1=1"
        Some(name) = filters.name => {
          "AND name LIKE"?(name)
        }
        "ORDER BY name"
        l = PAGE_SIZE => {
            "LIMIT" ?(l as i64)
        }
        s = skip => {
            "OFFSET" ?(s as i64)
        }
    )
        .fetch_all(&mut *db)
        .await
        .map(Json)
        .map_err(|_| Custom(Status::InternalServerError, String::from("Failed loading authors.")))
}

#[derive(Deserialize)]
pub struct CreateAuthor<'r> {
    name: &'r str,
}

#[rocket::post("/", format = "application/json", data = "<input>")]
pub async fn create_author<'r>(pool: &rocket::State<PgPool>, input: Json<CreateAuthor<'r>>) -> Res<Author> {
    let mut db = acquire_db(pool).await?;

    InsertAuthor {
        name: input.name.to_string()
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
