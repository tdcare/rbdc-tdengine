use futures_core::future::BoxFuture;
use rbdc::db::{ConnectOptions, Connection};
use rbdc::db::{Driver, Placeholder};
use rbdc::{Error, impl_exchange};
use crate::connection::TaosConnection;
use crate::options::TaosConnectOptions;


#[derive(Debug)]
pub struct TaosDriver {}

impl Driver for TaosDriver{
    fn name(&self) -> &str {
        "Taos"
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        Box::pin(async move {
            unimplemented!();
        })
    }

    fn connect_opt<'a>(&'a self, opt: &'a dyn ConnectOptions) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        let opt = opt.downcast_ref::<TaosConnectOptions>().unwrap();
        Box::pin(async move {
            let conn = TaosConnection::establish(opt).await?;
            Ok(Box::new(conn) as Box<dyn Connection>)
        })
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        Box::new(TaosConnectOptions::default())
    }
}
impl Placeholder for TaosDriver {
    fn exchange(&self, sql: &str) -> String {
        impl_exchange(":",1,sql)
    }
}
impl TaosDriver{
    pub fn pub_exchange(&self, sql: &str) -> String{
        self.exchange(sql)
    }
}