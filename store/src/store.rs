use crate::config::Config;
use diesel::ConnectionResult;
use diesel_async::pooled_connection::bb8::{Pool, PooledConnection};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::ManagerConfig;
use diesel_async::AsyncPgConnection;
use futures_util::future::BoxFuture;
use futures_util::FutureExt;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;

pub struct Store {
    pub pool: Pool<AsyncPgConnection>,
}

impl Store {
    pub async fn new() -> Self {
        let config = Config::default();
        
        let mut manager_config = ManagerConfig::default();
        // Use our native-tls setup function
        manager_config.custom_setup = Box::new(establish_connection);

        let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
            config.db_url, 
            manager_config
        );

        let pool = Pool::builder()
            .max_size(10)
            .build(mgr)
            .await
            .expect("Failed to create database pool");

        Self { pool }
    }

    pub async fn get_conn(&self) -> PooledConnection<'_, AsyncPgConnection> {
        self.pool.get().await.expect("Failed to get connection from pool")
    }
}

fn establish_connection(config: &str) -> BoxFuture<'_, ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        // 1. Create a native-tls connector
        // This automatically uses the Windows Certificate Store
        let native_connector = TlsConnector::builder()
            .build()
            .map_err(|e| diesel::ConnectionError::BadConnection(e.to_string()))?;
        
        let tls = MakeTlsConnector::new(native_connector);
        
        // 2. Connect using tokio-postgres
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| diesel::ConnectionError::BadConnection(e.to_string()))?;

        // 3. Convert the tokio-postgres client into a Diesel connection
        AsyncPgConnection::try_from_client_and_connection(client, conn).await
    };
    fut.boxed()
}