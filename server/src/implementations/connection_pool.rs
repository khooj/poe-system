use diesel::connection::AnsiTransactionManager;
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
use diesel::r2d2::{ManageConnection, Pool, PooledConnection};
use diesel::result::Error;
use diesel::sql_types::HasSqlType;
use diesel::{backend::UsesAnsiSavepointSyntax, connection::SimpleConnection};

pub struct ConnectionPool<T>
where
    T: ManageConnection,
{
    pub pool: Pool<T>,
}

impl<T> Clone for ConnectionPool<T>
where
    T: ManageConnection,
{
    fn clone(&self) -> ConnectionPool<T> {
        ConnectionPool {
            pool: self.pool.clone(),
        }
    }
}

impl<T> ConnectionPool<T>
where
    T: ManageConnection,
{
    fn get_conn(&self) -> Result<PooledConnection<T>, Error> {
        match self.pool.get() {
            Ok(k) => Ok(k),
            Err(_) => Err(Error::RollbackTransaction),
        }
    }
}

impl<T> SimpleConnection for ConnectionPool<T>
where
    T: ManageConnection,
    T::Connection: Connection + Send + 'static,
{
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        let conn = self.get_conn()?;

        conn.batch_execute(query)
    }
}

impl<C> Connection for ConnectionPool<C>
where
    C: ManageConnection,
    C::Connection: Connection<TransactionManager = AnsiTransactionManager> + Send + 'static,
    <C::Connection as Connection>::Backend: UsesAnsiSavepointSyntax,
{
    type Backend = <C::Connection as Connection>::Backend;
    // TODO: implement type with Drop trait to correctly use pooled connection and 
    // transaction manager in self.transaction_manager()?
    type TransactionManager = <C::Connection as Connection>::TransactionManager;

    fn establish(_: &str) -> ConnectionResult<Self> {
        Err(ConnectionError::BadConnection(String::from(
            "Cannot directly establish a pooled connection",
        )))
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        let conn = self.get_conn()?;
        conn.execute(query)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend> + QueryId,
        Self::Backend: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Self::Backend>,
    {
        let conn = self.get_conn()?;
        conn.query_by_index(source)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: QueryFragment<Self::Backend> + QueryId,
        U: QueryableByName<Self::Backend>,
    {
        let conn = self.get_conn()?;
        conn.query_by_name(source)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Self::Backend> + QueryId,
    {
        let conn = self.get_conn()?;
        conn.execute_returning_count(source)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        unimplemented!();
        // match self.get_conn() {
        //     Ok(k) => k.
        //     Err(e) => {
        //         panic!("cant get connection for transaction manager: {}", e)
        //     }
        // }
    }
}
