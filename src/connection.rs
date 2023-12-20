use std::collections::HashMap;
use futures_core::future::BoxFuture;
use rbdc::db::{Connection, ExecResult, Row};
use rbdc::Error;
use rbs::Value;
use std::sync::Arc;
use futures_util::TryFutureExt;
use rbs::value::map::ValueMap;
use taos::sync::*;
use taos::ColumnView;
// use taos_query::common::raw::rows::RowView;
// use taos_query::AsyncFetchable;

use crate::driver::TaosDriver;
use crate::encode::*;

use crate::options::TaosConnectOptions;
use crate::rows::{TaosColumn, TaosData, TaosRow};

// pub trait fields_count{
//     fn get_fields_count(&self) -> i32;
// }
//
// impl fields_count for ResultSet {
//     fn get_fields_count(&self) -> i32 {
//         self.num_of_fields() as i32
//     }
// }

#[derive(Clone)]
pub struct TaosConnection {
    pub conn: Arc<Taos>,
}

impl Connection for TaosConnection{
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        // let sql:String = TaosDriver {}.pub_exchange(sql);
        let mut sql=sql.to_string();

        Box::pin(async move {
            sql=sql_replacen(sql,params);
            log::debug!("将要执行的sql:{}",sql);

            let mut results = vec![];

            if sql.eq("begin") || sql.eq("commit") || sql.eq("rollback"){
                log::warn!("不支持事务相关操作,直接返回");
                return Ok(results)
            }

            let mut q = self.conn.query(sql).map_err(|e| Error::from(e.to_string()))?;

            if q.fields().len()==0{
                 return Ok(results)
             }
            let  fields=q.fields();

            let mut columns = vec![];
            for field in fields {
                columns.push(TaosColumn{
                    name: field.name().to_string(),
                    column_type: field.ty() });
            }
            for row in q.rows(){
                let row_view = row.map_err(|e| Error::from(e.to_string()))?;
                let mut datas =vec![];
                for (name, value) in row_view {
                    datas.push(TaosData {
                        value: Some(format!("{}",value)),
                        colunm_name: name.to_string(),
                    });

                }
                let taos_row = TaosRow {
                    columns: Arc::new(columns.clone()),
                    datas: datas,
                };
                results.push(Box::new(taos_row) as Box<dyn Row>);
            }

            Ok(results)
        })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        // log::debug!("sql={}",sql);
        let mut sql=sql.to_string();
        Box::pin(async move {
            // let mut p = Vec::with_capacity(params.len());
            // for x in params {
            //     x.encode(&mut p).map_err(|e| Error::from(e.to_string()))?
            // }
            // let mut stmt = Stmt::init(&self.conn).map_err(|e| Error::from(e.to_string()))?;
            // stmt.prepare(sql).map_err(|e| Error::from(e.to_string()))?;
            //
            // let rows = stmt.bind(&p).map_err(|e| Error::from(e.to_string()))?
            //     .add_batch().map_err(|e| Error::from(e.to_string()))?
            //     .execute().map_err(|e| Error::from(e.to_string()))?;
            sql=sql_replacen(sql,params);
            log::debug!("将要执行sql:{}",sql);
            if sql.eq("begin") || sql.eq("commit") || sql.eq("rollback"){
               log::warn!("不支持事务相关操作，直接返回");
               return  Ok(ExecResult {
                    rows_affected: 0 as u64,
                    last_insert_id: Value::Null,
                })
            }
           // let rows=tokio::runtime::Runtime::block_on(self.conn.exec(sql));
           let rows= self.conn.exec(sql).map_err(|e| Error::from(e.to_string()))?;
           // let handle=  tokio::runtime::Handle::try_current().unwrap();
            // let mut runtime =  tokio::runtime::current_thread();

            // let future = async move {
            //    return   self.conn.clone().exec(sql).map_err(|e| Error::from(e.to_string()));
            // };
           //  let s=&self.conn;
           // let rows= tokio::task::spawn_blocking(move || {
           //    let rows= s.exec(sql).map_err(|e| Error::from(e.to_string()));
           //    rows
           // }).await.unwrap().unwrap();
           // let rows= runtime.block_on(future)?;

           //  let rows=rows.map_err(|e| Error::from(e.to_string()))?;

          return   Ok(ExecResult {
                rows_affected: rows as u64,
                last_insert_id: Value::Null,
            })
        })
        }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async move {
            Ok(())
        })
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        Box::pin(async move {
            Ok(())
        })
    }
}

impl TaosConnection {
    pub async fn establish(opt: &TaosConnectOptions) -> Result<Self, Error> {

      let builder=  TaosBuilder::from_dsn(opt.dsn.clone())
                 .map_err(|e| Error::from(e.to_string()));

        match builder.map(|b|b.build()){
            Ok(taos) => {
                match  taos.map_err(|e|Error::from(e.to_string())){
                    Ok(conn) => {
                        Ok(Self{ conn: Arc::new(conn) })
                    }
                    Err(e) => {Err(e)}
                }
                },

            Err(e) => {Err(e)}
        }

    }
}
