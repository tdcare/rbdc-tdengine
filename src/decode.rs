use std::collections::HashMap;
use bigdecimal::BigDecimal;
// use oracle::sql_type::OracleType;
// use taos_query::prelude::ColumnView;
use taos::ColumnView;
use taos::Ty;
use std::sync::Arc;
use rbdc::{datetime::FastDateTime, Error};
use rbs::Value;
use std::str::FromStr;
use rbdc::datetime::DateTime;
use rbdc::timestamp::Timestamp;
use crate::rows::TaosColumn;
use crate::rows::TaosData;

pub trait Decode {
    fn decode(row: &TaosData,columns:&Arc<Vec<TaosColumn>>) -> Result<Value, Error>;
}

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
                return Ok(Value::I32(value.parse().unwrap()));
            }
            Some(Ty::SmallInt) => {
                return Ok(Value::I32(value.parse().unwrap()));
            }
            Some(Ty::Int) => {
                return Ok(Value::I32(value.parse().unwrap()));
            }
            Some(Ty::BigInt) => {
                return Ok(Value::I64(value.parse().unwrap()));

            }
            Some(Ty::UTinyInt) => {
                return Ok(Value::U32(value.parse().unwrap()));

            }
            Some(Ty::USmallInt) => {
                return Ok(Value::U32(value.parse().unwrap()));

            }
            Some(Ty::UInt) => {
                return Ok(Value::U32(value.parse().unwrap()));

            }
            Some(Ty::UBigInt) => {
                return Ok(Value::U64(value.parse().unwrap()));

            }
            Some(Ty::Float) => {
                return Ok(Value::F32(value.parse().unwrap()));

            }
            Some(Ty::Double) => {
                return Ok(Value::F64(value.parse().unwrap()));

            }
            Some(Ty::Timestamp) => {
               // let date=FastDateTime::from_str(&value).unwrap().unix_timestamp_millis();
                let timestamp=Timestamp::from_str(&value).unwrap();
                // let tv=TV::new("Timestamp",Value::I64(date));
               return  Ok(Value::from(timestamp));


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