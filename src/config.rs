use dotenv::dotenv;
use std::env;

pub fn get_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE must not be empty")
}
