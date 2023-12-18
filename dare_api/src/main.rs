#[macro_use]
extern crate dotenv_codegen;

mod setup;

use entity::dare;
use rocket::{serde::json::Json, *};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use setup::establish_connection;

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}

impl From<DbErr> for ErrorResponder {
    fn from(err: DbErr) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}

impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, API!"
}

#[get("/dares")]
async fn get_dares(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let all_dares = dare::Entity::find()
        .all(db)
        .await
        .map_err(ErrorResponder::from)?
        .into_iter()
        .map(|dare| dare.description)
        .collect::<Vec<String>>();

    Ok(Json(all_dares))
}

#[post("/dares?<title>&<description>&<author>")]
async fn create_dare(
    db: &State<DatabaseConnection>,
    title: &str,
    description: &str,
    author: &str,
) -> Result<(), ErrorResponder> {
    let db = db as &DatabaseConnection;

    let new_dare = dare::ActiveModel {
        title: ActiveValue::Set(title.to_owned()),
        description: ActiveValue::Set(description.to_owned()),
        author: ActiveValue::Set(author.to_owned()),
        ..Default::default()
    };

    let new_dare: dare::Model = new_dare.insert(db).await?;
    println!("Created new dare: {:?}", new_dare);

    Ok(())
}

#[launch]
async fn rocket() -> _ {
    let db = match establish_connection().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    rocket::build()
        .manage(db)
        .mount("/", routes![index, get_dares, create_dare])
}
