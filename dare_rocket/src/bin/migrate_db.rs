#[macro_use]
extern crate dotenv_codegen;

use std::process::{Command, Output};

use dotenv::from_filename;
use futures::executor::block_on;
use sea_orm::DbErr;

async fn run() -> Result<Output, DbErr> {
    let db_url = dotenv!("DATABASE_URL");
    println!("DATABASE_URL: {}", db_url);
    let output = Command::new("sea-orm-cli")
        .args(["migrate", "refresh"])
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    if output.status.success() {
        let success: String = String::from_utf8_lossy(&output.stdout).to_string();
        println!("{}", success);
    } else {
        let error: String = String::from_utf8_lossy(&output.stderr).to_string();
        println!("{}", error);
    }

    Ok(output)
}

#[tokio::main]
async fn main() {
    from_filename(".env").ok();
    println!("{}", "Migrating DB");
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
