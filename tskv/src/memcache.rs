use crate::error::Result;
use flatbuffers::Push;
use futures::future::ok;
use protos::models::FieldType;
use std::{borrow::BorrowMut, collections::HashMap, rc::Rc};

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Copy)]
pub struct DataCell<T> {
    pub ts: u64,
    pub val: T,
}

pub type Byte = Vec<u8>;
pub type U64Cell = DataCell<u64>;
pub type I64Cell = DataCell<i64>;
pub type StrCell = DataCell<Byte>;
pub type F64Cell = DataCell<f64>;
pub type BoolCell = DataCell<bool>;

#[derive(Debug, Clone)]
pub enum DataType {
    U64(U64Cell),
    I64(I64Cell),
    Str(StrCell),
    F64(F64Cell),
    Bool(BoolCell),
}

impl DataType {
    pub fn timestamp(&self) -> u64 {
        match *self {
            DataType::U64(U64Cell { ts, .. }) => ts,
            DataType::I64(I64Cell { ts, .. }) => ts,
            DataType::Str(StrCell { ts, .. }) => ts,
            DataType::F64(F64Cell { ts, .. }) => ts,
            DataType::Bool(BoolCell { ts, .. }) => ts,
        }
    }
}

#[allow(dead_code)]
pub struct MemEntry {
    ts_min: u64,
    ts_max: u64,
    field_type: FieldType,
    cells: Vec<DataType>,
}

#[allow(dead_code)]
pub struct MemCache {
    // partiton id
    tf_id: u32,
    //wal seq number
    seq_no: u64,
    //max mem buffer size convert to immcache
    max_buf_size: u64,
    //block <filed_id, buffer>
    //filed_id contain the field type
    data_cache: HashMap<u64, MemEntry>,
    //current size
    cache_size: u64,
}

impl MemCache {
    pub fn new(tf_id: u32, max_size: u64, seq: u64) -> Self {
        let cache = HashMap::new();
        Self {
            tf_id,
            max_buf_size: max_size,
            data_cache: cache,
            seq_no: seq,
            cache_size: 0,
        }
    }
    pub fn insert_raw(
        &mut self,
        seq: u64,
        filed_id: u64,
        ts: u64,
        field_type: FieldType,
        val: u64,
    ) -> Result<()> {
        self.seq_no = seq;
        match field_type {
            FieldType::Unsigned => {
                let data = DataType::U64(U64Cell { ts, val });
                self.insert(filed_id, data);
            }
            // FieldType::Integer(t) =>{
            //     self.insert(filed_id, ts, val)
            // },
            // FieldType::Float(t) =>{},
            // FieldType::String => {},
            // FieldType::Boolean => {},
            _ => todo!(),
        };
        Ok(())
    }
    pub fn insert(&mut self, filed_id: u64, val: DataType) {
        let entry = self.data_cache.get_mut(&filed_id);
        let ts = val.timestamp();
        if let Some(item) = entry {
            if item.ts_max < ts {
                item.ts_max = ts;
            }
            if item.ts_min > ts {
                item.ts_min = ts
            }
            item.cells.push(val);
        }
    }

    pub fn flush() -> Result<()> {
        Ok(())
    }

    pub fn is_full(&self) -> bool {
        return self.cache_size >= self.max_buf_size;
    }
}
