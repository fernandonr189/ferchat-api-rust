use crate::models::traits::Insertable;
use mysql::*;
pub fn create_pool() -> Pool {
    let url = "mysql://root:password@localhost:3306/users";
    let pool = Pool::new(url).expect("Failed to create pool.");
    pool
}

pub fn insert<'r>(pool: &Pool, object: &dyn Insertable) -> Result<bool> {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get connection.");
    let result = object.insert(&mut conn);
    result
}
