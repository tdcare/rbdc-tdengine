use bigdecimal::BigDecimal;
use rbatis::{Rbatis, py_sql};
use rbdc::{datetime::FastDateTime};
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use rbs::Value;
use fastdate::DateTime;
use chrono::DateTime as ChronoDateTime;
use rbdc_tdengine::driver::TaosDriver;
use rbdc_tdengine::options::TaosConnectOptions;
use rbatis::Error;
use rbatis::executor::Executor;
use serde_json::json;

#[macro_use]
extern crate rbatis;
extern crate rbdc;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct Record {
    // deserialize timestamp to chrono::DateTime<Local>
    ts: FastDateTime,
    // float to f32
    current: Option<f32>,
    // int to i32
    voltage: Option<i32>,
    phase: Option<f32>,
}

crud!(Record{},"meters");
#[py_sql(
"`INSERT INTO `
   ` #{tbname} USING meters TAGS(2,'California.SanFrancisco')`
   `  VALUES (#{dt}, 10.30000, 219, 0.31000)`
"
)]
async fn insert_to_meters(rb: &mut dyn Executor,tbname:&str,dt:FastDateTime)->(){
    impled!()
}


#[tokio::main]
async fn main()-> anyhow::Result<()> {
    fast_log::init(fast_log::Config::new().console()).expect("");
    let mut rb = Rbatis::new();
    rb.init_opt(
        TaosDriver {},
        TaosConnectOptions {
            dsn: "taos+ws://localhost:6041/test".to_string(),
        },
    )
        .expect("rbatis link database fail");
    
    // let ins=rb.exec("INSERT INTO tb1 VALUES (?,?) ", vec![Value::Ext("Timestamp",Box::new(Value::I64(1677859610000))),Value::I64(90)]).await;
    // let q=rb.query("select * from tb1", vec![]).await;
   let record=Record{
       ts: FastDateTime::now(),
       current: Some(43.0),
       voltage: Some(44),
       phase: Some(45.0),
   };

   // let rows=Record::insert(&mut rb,&record).await.expect("insert failed");
   //  println!("插入了{}行数据",rows.rows_affected);
    let dt=FastDateTime::now();
    println!("dt={}",dt);
    insert_to_meters(&mut rb,"d1001",dt).await;
    let rows=Record::select_all(&mut rb).await?;
       println!("超级表:{}",json!(rows));
Ok(())
}