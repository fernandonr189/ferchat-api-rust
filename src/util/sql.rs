use crate::models::traits::Insertable;
use mysql::Error;
use mysql::*;
use prelude::{FromRow, Queryable};

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

pub fn query<'r, T: FromRow>(pool: &Pool, query: &'r str) -> Result<Vec<T>, Error> {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get a connection");
    let selected_objects = conn.query(query);
    match selected_objects {
        Ok(objects) => Ok(objects),
        Err(e) => Err(e),
    }
}
