use crate::{
    models::{Author, InsertAuthor},
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
pub struct AuthorFilters<'r> {
    name: Option<&'r str>,
}

#[rocket::get("/?<page>&<filters..>", format = "application/json")]
async fn list_authors<'r>(
    pool: &rocket::State<PgPool>,
    filters: AuthorFilters<'r>,
    page: Option<u32>,
) -> Res<Vec<Author>> {
    let mut db = acquire_db(pool).await?;

    let skip: u32 = match page {
        None | Some(0) => 0,
        Some(page) => (page - 1) * PAGE_SIZE,
    };

    ormx::conditional_query_as!(
        Author,
        "SELECT * FROM authors"
        "WHERE 1=1"
        Some(name) = filters.name => {
          "AND name LIKE"?(format!("%{}%", name))
        }
        "ORDER BY name"
        "LIMIT" ?(PAGE_SIZE as i64)
        "OFFSET" ?(skip as i64)
    )
    .fetch_all(&mut *db)
    .await
    .map(Json)
    .map_err(|_| {
        Custom(
            Status::InternalServerError,
            String::from("Failed loading authors."),
        )
    })
}

#[derive(Deserialize)]
pub struct CreateAuthor<'r> {
    name: &'r str,
}

#[rocket::post("/", format = "application/json", data = "<input>")]
async fn create_author<'r>(
    pool: &rocket::State<PgPool>,
    input: Json<CreateAuthor<'r>>,
) -> Res<Author> {
    let mut db = acquire_db(pool).await?;

    InsertAuthor {
        name: input.name.to_string(),
    }
    .insert(&mut *db)
    .await
    .map(Json)
    .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

#[derive(Deserialize)]
struct UpdateAuthorBody<'r> {
    name: &'r str,
}

#[rocket::put("/<author_id>", format = "application/json", data = "<input>")]
async fn update_author<'r>(
    pool: &rocket::State<PgPool>,
    author_id: i32,
    input: Json<UpdateAuthorBody<'r>>,
) -> Res<Author> {
    let mut db = acquire_db(pool).await?;

    let mut author = Author::get(&mut *db, author_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Custom(
                Status::NotFound,
                format!("author_id {} not found", author_id),
            ),
            _ => Custom(
                Status::InternalServerError,
                String::from("Error fetching from database."),
            ),
        })?;

    author.name = input.name.to_string();
    author
        .update(&mut *db)
        .await
        .map(|_| Json(author))
        .map_err(|_| Custom(Status::InternalServerError, String::from("Error inserting")))
}

pub fn author_routes() -> Vec<Route> {
    rocket::routes![list_authors, create_author, update_author]
}
