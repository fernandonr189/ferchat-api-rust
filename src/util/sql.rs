use mysql::Error;
use mysql::*;
use prelude::{FromRow, Queryable};

pub fn create_pool() -> Result<Pool, mysql::Error> {
    let url = "mysql://root:password@localhost:3306/users";
    Pool::new(url)
}

pub fn insert<'r>(pool: &Pool, query: &'r str) -> Result<(), Error> {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get a connection");
    conn.query_drop(query)
}

pub fn query_vec<'r, T: FromRow>(pool: &Pool, query: &'r str) -> Result<Vec<T>, Error> {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get a connection");
    conn.query(query)
}

pub fn query_element<'r, T: FromRow>(pool: &Pool, query: &'r str) -> Result<Option<T>, Error> {
    let mut conn: PooledConn = pool.get_conn().expect("Failed to get a connection");
    conn.query_first(query)
}
