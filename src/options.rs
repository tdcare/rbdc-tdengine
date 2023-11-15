use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection};
use rbdc::Error;
use serde::{Deserialize, Serialize};
use std::any::Any;

use crate::connection::TaosConnection;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaosConnectOptions {
    pub dsn: String,
}

impl ConnectOptions for TaosConnectOptions{
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            let v = TaosConnection::establish(self)
                .await
                .map_err(|e| Error::from(e.to_string()))?;
            Ok(Box::new(v) as Box<dyn Connection>)
        })
    }

    fn set_uri(&mut self, url: &str) -> Result<(), Error> {
        *self = TaosConnectOptions::from_str(url)?;
        Ok(())
    }

    // fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
    //     self
    // }
}
impl Default for TaosConnectOptions {
    fn default() -> Self {
        Self {
            dsn: "taos+ws://localhost:6041".to_owned(),
        }
    }
}

impl TaosConnectOptions {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        serde_json::from_str(s).map_err(|e| Error::from(e.to_string()))
    }
}