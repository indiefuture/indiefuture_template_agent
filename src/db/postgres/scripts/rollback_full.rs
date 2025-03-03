use dotenvy::dotenv;
use inquire::Confirm;

use degen_sql::db::postgres::postgres_db::{Database, DatabaseCredentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

       dotenv().ok();
       
    let ans = Confirm::new("Are you sure you want to roll back?")
        .with_default(false)
        .prompt();

    match ans {
        Ok(true) => {
                let db_conn_url = std::env::var("DB_CONN_URL").expect(" DB_CONN_URL must be set in env ");


          let mut database = Database::new(db_conn_url, None) ?;

            let _migration = database.rollback_full().await?;

            println!("Rollback complete");
        }
        Ok(false) => {
            println!("Rollback operation cancelled");
        }
        Err(_) => {
            println!("Rollback operation cancelled");
        }
    }

    Ok(())
}
