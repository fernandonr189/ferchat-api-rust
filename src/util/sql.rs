use crate::models::traits::Repository;
use mysql::*;
pub fn create_pool() -> Pool {
    let url = "mysql://root:password@localhost:3306/users";
    let pool = Pool::new(url).expect("Failed to create pool.");
    pool
}

pub fn insert<'r>(pool: &Pool, object: &dyn Repository) {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get connection.");
    object.insert(&mut conn).expect("Failed to insert user.");
}
