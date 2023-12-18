#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

const DATABASE_URL: &str = dotenv!("POSTGRES_URL");
const DB_NAME: &str = dotenv!("POSTGRES_DB");

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = &match db.get_database_backend() {
        // This isn't used. The DB is postgres
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }

        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }

        // This isn't used. The DB is postgres
        DbBackend::Sqlite => db,
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("{}", dotenv!("POSTGRES_URL"));
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
