#[macro_use]
extern crate dotenv_codegen;

mod setup;

use rocket::*;
use setup::set_up_db;

#[get("/")]
async fn index() -> &'static str {
    "Hello, API!"
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    rocket::build().manage(db).mount("/", routes![index])
}
