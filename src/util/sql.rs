use mysql::Error;
use mysql::*;
use prelude::{FromRow, Queryable};

pub fn create_pool() -> Result<Pool, Error> {
    let url = "mysql://root:password@localhost:3306/users?pool_min=0&pool_max=1000";
    Pool::new(url)
}

pub fn insert(pool: &Pool, query: &str) -> Result<(), Error> {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get a connection");
    conn.query_drop(query)
}

pub fn query_vec<T: FromRow>(pool: &Pool, query: &str) -> Result<Vec<T>, Error> {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get a connection");
    conn.query(query)
}

pub fn query_element<T: FromRow>(pool: &Pool, query: &str) -> Result<Option<T>, Error> {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get a connection");
    conn.query_first(query)
}
