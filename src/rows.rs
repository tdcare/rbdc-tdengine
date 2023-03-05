
use std::sync::{Arc};
// use oracle::sql_type::OracleType;
use taos::Ty;
use rbdc::db::{Row, MetaData};
use rbs::Value;
use crate::decode::Decode;

#[derive(Debug,Clone)]
pub struct TaosColumn{
    pub name:String,
    pub column_type:Ty
}

#[derive(Debug)]
pub struct TaosMetaData(pub Arc<Vec<TaosColumn>>);

impl MetaData for TaosMetaData {
    fn column_len(&self) -> usize {
        self.0.len()
    }

    fn column_name(&self, i: usize) -> String {
        self.0[i].name.to_string()
    }

    fn column_type(&self, i: usize) -> String {
        format!("{:?}", self.0[i].column_type)
    }
}

#[derive(Debug,Clone)]
pub struct TaosData{
    pub value:Option<String>,
    pub colunm_name:String
}

#[derive(Debug)]
pub struct TaosRow {
    pub columns: Arc<Vec<TaosColumn>>,
    pub datas: Vec<TaosData>,
}
unsafe impl Send for TaosRow{}
unsafe impl Sync for TaosRow{}
impl Row for TaosRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        Box::new(TaosMetaData(self.columns.clone()))
    }

    fn get(&mut self, i: usize) -> Result<Value, rbdc::Error> {
        Value::decode(
            &self.datas[i],
                 &self.columns,
        )
    }
}