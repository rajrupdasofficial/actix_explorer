use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}

// Add a method to get a connection from the pool
pub fn get_connection(pool: &DbPool) -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
    pool.get()
        .expect("Failed to get database connection from pool")
}
