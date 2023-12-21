#[macro_use]
extern crate dotenv_codegen;

mod setup;

use entity::dare::{ self };
use rocket::{ serde::json::Json, * };
use sea_orm::{ ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait };
use ::serde::Serialize;
use setup::establish_connection;
use rocket::http::Method;
use rocket_cors::{ AllowedOrigins, CorsOptions };

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}

#[derive(Debug, Serialize)]
struct Dare {
    id: i32,
    title: String,
    description: String,
    author: String,
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

#[options("/")]
async fn cors_preflight() -> Result<&'static str, ErrorResponder> {
    Ok("OK")
}
#[get("/")]
async fn index() -> Result<Json<Dare>, ErrorResponder> {
    let dare = {
        Dare {
            id: 1,
            title: "Test".to_owned(),
            description: "Test".to_owned(),
            author: "Test".to_owned(),
        }
    };
    Ok(Json(dare))
}

#[get("/dares")]
async fn get_dares(db: &State<DatabaseConnection>) -> Result<Json<Vec<Dare>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let all_dares = dare::Entity
        ::find()
        .all(db).await
        .map_err(ErrorResponder::from)?
        .into_iter()
        .map(|dare| Dare {
            id: dare.id,
            title: dare.title,
            description: dare.description,
            author: dare.author,
        })
        .collect::<Vec<Dare>>();
    println!("Found dares: {:?}", all_dares);
    Ok(Json(all_dares))
}

#[post("/dares?<title>&<description>&<author>")]
async fn create_dare(
    db: &State<DatabaseConnection>,
    title: &str,
    description: &str,
    author: &str
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

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch].into_iter().map(From::from).collect()
        )
        .allow_credentials(true);

    rocket
        ::build()
        .attach(cors.to_cors().unwrap())
        .manage(db)
        .mount("/", routes![cors_preflight, index, get_dares, create_dare])
}
