use std::env;

use diesel::{PgConnection, r2d2, RunQueryDsl};
use diesel::r2d2::ConnectionManager;

use crate::entity;
use crate::schema::images::dsl::images;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap_or("postgres://localhost:5432/server-database".to_string());
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn fetch_images(&self) -> Vec<entity::Image> {
        let mut conn = self.pool.get().unwrap();
        images.load::<entity::Image>(&mut conn)
            .expect("Error loading all images")
    }
}
