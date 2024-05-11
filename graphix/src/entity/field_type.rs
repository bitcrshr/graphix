use time::PrimitiveDateTime;
use uuid::Uuid;
use serde_json::Value as JsonValue;
use crate::dialect::PostgresColumnType;


pub enum FieldType {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
    F32(f32),
    F64(f64),
    Boolean(bool),
    String(String),
    Time(PrimitiveDateTime),
    Uuid(Uuid),
    Bytes(Vec<u8>),
    Json(JsonValue),
    Enum(Vec<String>),
    Other { sql_type: PostgresColumnType, value: JsonValue },
}