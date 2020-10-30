use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use diesel::PgConnection;
use crate::config::Config;

// no one wants to write a type out this long
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// creates a new connection manager pool for async database calls
pub fn new_pool(config: &Config) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(&config.db_address);
    match config.pool_limit {
        Some(limit) => r2d2::Pool::builder()
            .max_size(limit)
            .build(manager)
            .expect("Unable to build pool"),
        None => r2d2::Pool::builder()
            .build(manager)
            .expect("Unable to build pool"),
    }
}
