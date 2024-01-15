use std::env;

use diesel::{BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, r2d2, RunQueryDsl};
use diesel::r2d2::ConnectionManager;

use crate::entity;

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
        use crate::schema::images::dsl::images;

        let mut conn = self.pool.get().unwrap();
        images.load::<entity::Image>(&mut conn)
            .expect("Error loading all images")
    }

    pub fn save_image(&self, image: entity::NewImage) {
        use crate::schema::images::dsl::images;

        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(images)
            .values(image)
            .execute(&mut conn)
            .expect("Error inserting a new image");
    }

    pub fn is_duplicated_image(&self, image: entity::NewImage) -> bool {
        use crate::schema::images::dsl::*;

        let mut conn = self.pool.get().unwrap();
        let duplicated = images.filter(
            title.eq(image.title)
                .and(url.eq(image.url))
                .and(alt_text.eq(image.alt_text))
        )
            .first::<entity::Image>(&mut conn)
            .optional()
            .expect("Error checking if image exists");

        duplicated.is_some()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
