use mysql::PooledConn;

pub trait Repository {
    fn insert(&self, conn: &mut PooledConn) -> Result<u64, mysql::Error>;
}
