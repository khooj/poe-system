use diesel::backend::Backend;
use diesel::connection::AnsiTransactionManager;
use diesel::debug_query;
use diesel::deserialize::QueryableByName;
use diesel::prelude::*;
use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
use diesel::sql_types::HasSqlType;
use diesel::{backend::UsesAnsiSavepointSyntax, connection::SimpleConnection};
use std::default::Default;
use std::ops::Deref;
use tracing::{event, Level};

#[derive(Clone)]
pub struct WrappedConnectionPool<C: Connection>(C);

impl<C: Connection> WrappedConnectionPool<C> {
    pub fn new(conn: C) -> Self {
        WrappedConnectionPool(conn)
    }
}

impl<C: Connection> Deref for WrappedConnectionPool<C> {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C> SimpleConnection for WrappedConnectionPool<C>
where
    C: Connection + Send + 'static,
{
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        event!(Level::DEBUG, "{}", query);
        self.0.batch_execute(query)
    }
}

impl<C: Connection> Connection for WrappedConnectionPool<C>
where
    C: Connection<TransactionManager = AnsiTransactionManager> + Send + 'static,
    C::Backend: Backend + UsesAnsiSavepointSyntax,
    <C::Backend as Backend>::QueryBuilder: Default,
{
    type Backend = C::Backend;
    type TransactionManager = C::TransactionManager;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        Ok(WrappedConnectionPool(C::establish(database_url)?))
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        event!(Level::DEBUG, "{}", query);
        self.0.execute(query)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend> + QueryId,
        Self::Backend: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Self::Backend>,
    {
        let query = source.as_query();
        let debug_query = debug_query::<Self::Backend, _>(&query);
        event!(Level::DEBUG, "{}", debug_query);
        self.0.query_by_index(query)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: QueryFragment<Self::Backend> + QueryId,
        U: QueryableByName<Self::Backend>,
    {
        let debug_query = debug_query::<Self::Backend, _>(&source);
        event!(Level::DEBUG, "{}", debug_query);
        self.0.query_by_name(source)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Self::Backend> + QueryId,
    {
        let debug_query = debug_query::<Self::Backend, _>(&source);
        event!(Level::DEBUG, "{}", debug_query);
        self.0.execute_returning_count(source)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        self.0.transaction_manager()
    }
}
