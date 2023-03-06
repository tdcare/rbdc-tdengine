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
use crate::encode::Encode;

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
            for v in params {
                match v {
                    Value::Null => {}
                    // Value::Bool(_) => {
                    //     sql= sql.replacen("?", &*format!("{}", v), 1);
                    // }
                    // Value::I32(_) => {}
                    // Value::I64(_) => {}
                    // Value::U32(_) => {}
                    // Value::U64(_) => {}
                    // Value::F32(_) => {}
                    // Value::F64(_) => {}
                    Value::String(_) => {
                        sql= sql.replacen("?", &*format!("{}", v), 1);
                        sql= sql.replace("\"", "'");
                    }
                    // Value::Binary(_) => {}
                    // Value::Array(_) => {}
                    // Value::Map(_) => {}
                    Value::Ext(name, ext_v) => {
                        if name.eq("Timestamp") {
                            sql= sql.replacen("?", &*format!("{}", ext_v), 1);
                        }
                        if name.eq("DateTime"){
                            sql= sql.replacen("?", &*format!("{}", ext_v), 1);
                        }

                    }
                    _=>{
                        sql= sql.replacen("?", &*format!("{}", v), 1);
                    }
                }
            }
            let mut q = self.conn.query(sql).map_err(|e| Error::from(e.to_string()))?;
            let mut results = vec![];

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
                let row = row.map_err(|e| Error::from(e.to_string()))?;
                let mut datas =vec![];
                for (name, value) in row {
                    datas.push(TaosData {
                        value: Some(format!("{}",value)),
                        colunm_name: name.to_string(),
                    });

                }
                let row = TaosRow {
                    columns: Arc::new(columns.clone()),
                    datas: datas,
                };
                results.push(Box::new(row) as Box<dyn Row>);
            }

            Ok(results)
        })
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        // log::debug!("sql={}",sql);
        let mut sql=sql.to_string();
        Box::pin(async move {
            let mut p = Vec::with_capacity(params.len());
            for x in params {
                x.encode(&mut p).map_err(|e| Error::from(e.to_string()))?
            }
            let mut stmt = Stmt::init(&self.conn).map_err(|e| Error::from(e.to_string()))?;
            stmt.prepare(sql).map_err(|e| Error::from(e.to_string()))?;

            let rows = stmt.bind(&p).map_err(|e| Error::from(e.to_string()))?
                .add_batch().map_err(|e| Error::from(e.to_string()))?
                .execute().map_err(|e| Error::from(e.to_string()))?;
            Ok(ExecResult {
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
