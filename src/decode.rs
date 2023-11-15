use std::collections::HashMap;
use bigdecimal::BigDecimal;
// use oracle::sql_type::OracleType;
// use taos_query::prelude::ColumnView;
use taos::ColumnView;
use taos::Ty;
use std::sync::Arc;
use rbdc::{datetime::DateTime, Error};
use rbs::Value;
use std::str::FromStr;
use rbdc::timestamp::Timestamp;
use crate::rows::TaosColumn;
use crate::rows::TaosData;

pub trait Decode {
    fn decode(row: &TaosData,columns:&Arc<Vec<TaosColumn>>) -> Result<Value, Error>;
}
/// 将TDengine 返回的数据转换为 Rbatis 的Value 类型
impl Decode for Value {
    fn decode(row: &TaosData,columns:&Arc<Vec<TaosColumn>>) -> Result<Value, Error> {
        let s = row.value.as_ref();
        if s.is_none() {
            return Ok(Value::Null);
        }
        let row_name=row.colunm_name.clone();

        let mut ty=None;
        for column in columns.iter() {
            if column.name.eq(&row_name){
                ty=Some(column.column_type);
                break;
            }
        }

        let value = s.unwrap().clone();
        match ty {
            Some(Ty::Null) => {
                return Ok(Value::Null);
            }
            Some(Ty::Bool) => {
                return Ok(Value::Bool(value.parse().unwrap()));
            }
            Some(Ty::TinyInt) => {
                if let Ok(v)=value.parse::<i32>(){
                    return Ok(Value::I32(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::SmallInt) => {
                if let Ok(v)=value.parse::<i32>(){
                    return Ok(Value::I32(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::Int) => {
                if let Ok(v)=value.parse::<i32>(){
                    return Ok(Value::I32(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::BigInt) => {
                if let Ok(v)=value.parse::<i64>(){
                    return Ok(Value::I64(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::UTinyInt) => {
                if let Ok(v)=value.parse::<u32>(){
                    return Ok(Value::U32(v));
                }else {
                    return Ok(Value::Null);
                }

            }
            Some(Ty::USmallInt) => {
                if let Ok(v)=value.parse::<u32>(){
                    return Ok(Value::U32(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::UInt) => {
                if let Ok(v)=value.parse::<u32>(){
                    return Ok(Value::U32(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::UBigInt) => {
                if let Ok(v)=value.parse::<u64>(){
                    return Ok(Value::U64(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::Float) => {
                if let Ok(v)=value.parse::<f32>(){
                    return Ok(Value::F32(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::Double) => {
                if let Ok(v)=value.parse::<f64>(){
                    return Ok(Value::F64(v));
                }else {
                    return Ok(Value::Null);
                }
            }
            Some(Ty::Timestamp) => {
               // let date=FastDateTime::from_str(&value).unwrap().unix_timestamp_millis();
               //  let timestamp=Timestamp::from_str(&value).unwrap();
               //  // let tv=TV::new("Timestamp",Value::I64(date));
               // return  Ok(Value::from(timestamp));
               //  let date=FastDateTime::from_str(&value).unwrap().unix_timestamp_millis();
               //  return Ok(Value::Ext("Timestamp",Box::new(Value::I64(date))));
               //
                let datetime=DateTime::from_str(&value).unwrap();
                return Ok(Value::from(datetime));

            }
            Some(Ty::VarChar) => {
                return Ok(Value::String(value.parse().unwrap()));

            }
            Some(Ty::NChar) => {
                return Ok(Value::String(value.parse().unwrap()));

            }
            Some(Ty::Json) => {
                return Ok(Value::String(value.parse().unwrap()));

            }
            Some(Ty::VarBinary) => {
                return Ok(Value::Binary(value.into_bytes()));

            }
            Some(Ty::Decimal) => {
                return Ok(Value::Binary(value.into_bytes()));
            }
            Some(Ty::Blob) => {
                return Ok(Value::Binary(value.into_bytes()));

            }
            Some(Ty::MediumBlob) => {
                return Ok(Value::Binary(value.into_bytes()));

            }
            //TODO: more types!
            _ => return Ok(Value::String(value)),
        };


    }
}