pub mod users;

pub fn establish_connection() -> rusqlite::Result<rusqlite::Connection> {
    rusqlite::Connection::open("user_database.db")
}
