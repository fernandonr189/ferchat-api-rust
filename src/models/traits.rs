use mysql::PooledConn;

pub trait Insertable {
    fn insert(&self, conn: &mut PooledConn) -> Result<bool, mysql::Error>;
}
