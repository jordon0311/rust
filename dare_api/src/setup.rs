use migration::{DbErr, Migrator, MigratorTrait};
use sea_orm::*;

const DATABASE_URL: &str = dotenv!("DATABASE_URL");

// pub(super) async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
//     let db = Database::connect(DATABASE_URL).await?;

//     let db = match db.get_database_backend() {
//         // This isn't used. The DB is postgres
//         DbBackend::MySql => {
//             db.execute(Statement::from_string(
//                 db.get_database_backend(),
//                 format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
//             ))
//             .await?;

//             let url = format!("{}/{}", DATABASE_URL, DB_NAME);
//             Database::connect(&url).await?
//         }

//         DbBackend::Postgres => {
//             db.execute(Statement::from_string(
//                 db.get_database_backend(),
//                 format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
//             ))
//             .await?;
//             db.execute(Statement::from_string(
//                 db.get_database_backend(),
//                 format!("CREATE DATABASE \"{}\";", DB_NAME),
//             ))
//             .await?;

//             let url = format!("{}/{}", DATABASE_URL, DB_NAME);
//             Database::connect(&url).await?
//         }

//         // This isn't used. The DB is postgres
//         DbBackend::Sqlite => db,
//     };

//     Ok(db)
// }

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let db = Database::connect(DATABASE_URL).await.expect(
        "Could not connect to the database. Please check that the database is running and that the environment variables are set correctly.",
    );
    Migrator::up(&db, None)
        .await
        .expect("Could not run migrations");

    Ok(db)
}
