use std::str::FromStr;

// use bigdecimal::BigDecimal;
// use oracle::sql_type::ToSql;
// use taos_query::prelude::ColumnView;
use fastdate::DateTime;
use std::ops::Index;
use rbdc::Error;
use rbdc::json::Json;
use rbdc::timestamp::Timestamp;
use rbdc::types::*;
use rbdc::uuid::Uuid;
use rbs::Value;
// use taos::ColumnView;
//
// pub trait Encode {
//     fn encode(self, vec: & mut Vec<ColumnView>,) -> Result<(), Error>;
// }
// impl Encode for Value {
//     fn encode(self, vec: &mut Vec<ColumnView>) -> Result<(), Error> {
//         match self{
//             // Value::Ext(t, v) => match t {
//             //     "Date" => {
//             //         let s = v.as_str().unwrap_or_default();
//             //         let d = DateTime::from_str(s).unwrap();
//             //         // d.and_hms_milli_opt()
//             //
//             //         let cv= ColumnView::from_millis_timestamp(vec![d.unix_timestamp_millis()]);
//             //         vec.push(cv);
//             //     },
//             //     "DateTime" => {
//             //         let s = v.as_str().unwrap_or_default();
//             //         let d = DateTime::from_str(s).unwrap();
//             //         // d.and_hms_milli_opt()
//             //
//             //         let cv= ColumnView::from_millis_timestamp(vec![d.unix_timestamp_millis()]);
//             //         vec.push(cv);
//             //     }
//             //     "Time" => {
//             //         //TODO: need to fix this
//             //         let s = v.as_str().unwrap_or_default();
//             //         let d = DateTime::from_str(s).unwrap();
//             //         // d.and_hms_milli_opt()
//             //         let cv= ColumnView::from_millis_timestamp(vec![d.unix_timestamp_millis()]);
//             //         vec.push(cv);
//             //     }
//             //     "Decimal" => {
//             //         // let d = BigDecimal::from_str(&v.into_string().unwrap_or_default());
//             //         let d:f64=v.into_string().unwrap().parse().unwrap();
//             //         let cv=ColumnView::from_doubles(vec![d]);
//             //         vec.push(cv);
//             //     }
//             //     "Json" => {
//             //         return Err(Error::from("unimpl"));
//             //     }
//             //     "Timestamp" => {
//             //         let t = v.as_u64().unwrap_or_default() as i64;
//             //         let cv= ColumnView::from_millis_timestamp(vec![t]);
//             //         vec.push(cv);
//             //     }
//             //     "Uuid" => {
//             //         let d = v.into_string().unwrap();
//             //         let cv=ColumnView::from_nchar(vec![d]);
//             //         vec.push(cv);
//             //     }
//             //     _ => {
//             //         return Err(Error::from("unimpl"));
//             //     },
//             // }
//             Value::String(str)=>{
//                 let cv=ColumnView::from_nchar(vec![str]);
//                 vec.push(cv);
//             }
//             Value::I32(int)=>{
//                 let cv=ColumnView::from_ints(vec![int]);
//                 vec.push(cv);
//             }
//             Value::I64(int)=>{
//                 let cv=ColumnView::from_big_ints(vec![int]);
//                 vec.push(cv);
//             }
//             Value::F32(v)=>{
//                 let cv=ColumnView::from_floats(vec![v]);
//                 vec.push(cv);
//             }
//             Value::F64(v)=>{
//                 let cv=ColumnView::from_doubles(vec![v]);
//                 vec.push(cv);
//             }
//             Value::U32(v)=>{
//                 let cv=ColumnView::from_unsigned_ints(vec![v]);
//                 vec.push(cv);
//             }
//             Value::U64(v)=>{
//                 let cv=ColumnView::from_unsigned_big_ints(vec![v]);
//                 vec.push(cv);
//             }
//
//             //TODO: more types!
//             _=>{
//                 let cv=ColumnView::from_nchar(vec![self.to_string()]);
//                 vec.push(cv);
//             }
//         }
//         Ok(())
//     }
// }

/// 将sql 语名中的 ？ 替换 为Value 中的值
pub fn sql_replacen(mut sql:String,params: Vec<Value>)->String {
    for v in params {
        match v {
            // Value::Null => {}
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
                sql = sql.replacen("?", &*format!("{}", v), 1);
                // sql = sql.replace("\"", "'");
            }
            // Value::Binary(_) => {}
            // Value::Array(_) => {}
            // Value::Map(_) => {}
            Value::Ext(name, ext_v) => {
                if name.eq("Timestamp") {
                    let v=format!("{}",ext_v);
                    let v=v.parse::<u64>().unwrap_or_default();
                    sql = sql.replacen("?", &*format!("{}",v), 1);
                }
                if name.eq("DateTime"){
                    sql= sql.replacen("?", &*format!("{}", ext_v), 1);
                }
                if name.eq("Time"){
                    sql= sql.replacen("?", &*format!("{}", ext_v), 1);
                }
                if name.eq("DateTime"){
                    sql= sql.replacen("?", &*format!("{}", ext_v), 1);
                }
            }
            // Value::Map(mut m) => {
            //     //Ok(IsNull::Yes)
            //     println!("{}",m);
            //     let t = m.index("type").as_str().unwrap_or_default();
            //     if t != "" {
            //         match t {
            //             "Date" => {
            //                 let ext_v = m.rm("value");
            //                 sql = sql.replacen("?", &*format!("{}", ext_v), 1);
            //             }
            //             "DateTime" => {
            //                 let ext_v = m.rm("value");
            //                 sql = sql.replacen("?", &*format!("{}", ext_v), 1);
            //             }
            //             "Time" => {
            //                 let ext_v = m.rm("value");
            //                 sql = sql.replacen("?", &*format!("{}", ext_v), 1);
            //             }
            //             "Timestamp" => {
            //                 let ext_v = m.rm("value");
            //                 let ext_v_string=format!("{}", ext_v).replace("TS","");
            //                 sql = sql.replacen("?", &ext_v_string, 1);
            //             }
            //             "Decimal" => {
            //                 let ext_v = m.rm("value");
            //                 sql = sql.replacen("?", &*format!("{}", ext_v), 1);
            //             }
            //             "Json" => {
            //                 let ext_v = m.rm("value");
            //                 sql = sql.replacen("?", &*format!("{}", ext_v), 1);
            //             }
            //             "Uuid" => {
            //                 let ext_v = m.rm("value");
            //                 sql = sql.replacen("?", &*format!("{}", ext_v), 1);
            //             }
            //             _ => {
            //                 let ext_v = m.rm("value");
            //                 sql = sql.replacen("?", &*format!("{}", ext_v), 1);
            //             }
            //         }
            //     }
            // }
            _ => {
                sql = sql.replacen("?", &*format!("{}", v), 1);
            }
        }

    }
    sql = sql.replace("\"", "'");

    return sql;
}

#[cfg(test)]
mod test{
    use std::fmt::Debug;
    use std::str::FromStr;
    use fastdate::DateTime;
    use rbdc::date::Date;
    use rbdc::{datetime, time};
    use rbdc::timestamp::Timestamp;
    use rbs::Value;
    use taos::ColumnView;
    use crate::encode::sql_replacen;
    // use crate::encode::Encode;

    #[test]
    fn test_value(){
        let string_v=Value::String("测试".to_string());
        let timestamp_v=Value::Ext("Timestamp",Box::new(Value::I64(1677859610000)));
        println!("{},{}",timestamp_v,string_v);
        let mut  cvs:Vec<ColumnView>=vec![];
        // string_v.encode(&mut cvs);
        // timestamp_v.encode(&mut cvs);
        // for cv in cvs {
        //     println!("{}",cv.to_vec());
        // }
    }
    #[test]
    fn string_replacen(){
        let mut sql="select * from table where id=? and name=? and u32=? and bool=? timestamp<? and date>? and datetime<? and time=?".to_string();
        let vaules=vec![
                        Value::I64(10),
                        Value::String("测试".to_string()),
                        Value::U32(32),
                        Value::Bool(false),
                        Value::Ext("Timestamp",Box::new(Value::I64(1677859610000))),
                        Value::Ext("Date", Box::new(Value::String("2023-03-20".to_string()))),
                        // Value::from(Date::from_str("2023-03-20").unwrap()),
                        Value::Ext("DateTime", Box::new(Value::String(fastdate::DateTime::now().to_string()))),
                        // Value::from(datetime::DateTime::from(fastdate::DateTime::now())),
                        Value::Ext("Time", Box::new(Value::String("15:04:05.999999999".to_string()))),
                        // Value::from(rbdc::types::time::Time::from_str("15:04:05.999999999").unwrap()),



        ];
        sql=sql_replacen(sql,vaules);
        // sql=sql.replace("\"","'");
        println!("{}",sql);
    }
}
