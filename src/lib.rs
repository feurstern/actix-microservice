pub mod models;
pub mod schema;

use diesel::{pg::PgConnection, Connection, RunQueryDsl, SelectableHelper};

use models::role;
use schema;

pub fn establish_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
    dotenv.ok();
}
