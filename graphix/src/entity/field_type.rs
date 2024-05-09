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
impl From<i8> for FieldType {
    fn from(value: i8) -> Self {
        FieldType::I8(value)
    }
}
impl From<i16> for FieldType {
    fn from(value: i16) -> Self {
        FieldType::I16(value)
    }
}
impl From<i32> for FieldType {
    fn from(value: i32) -> Self {
        FieldType::I32(value)
    }
}
impl From<i64> for FieldType {
    fn from(value: i64) -> Self {
        FieldType::I64(value)
    }
}
impl From<isize> for FieldType {
    fn from(value: isize) -> Self {
        FieldType::ISize(value)
    }
}
impl From<u8> for FieldType {
    fn from(value: u8) -> Self {
        FieldType::U8(value)
    }
}
impl From<u16> for FieldType {
    fn from(value: u16) -> Self {
        FieldType::U16(value)
    }
}
impl From<u32> for FieldType {
    fn from(value: u32) -> Self {
        FieldType::U32(value)
    }
}
impl From<u64> for FieldType {
    fn from(value: u64) -> Self {
        FieldType::U64(value)
    }
}
impl From<usize> for FieldType {
    fn from(value: usize) -> Self {
        FieldType::USize(value)
    }
}
impl From<f32> for FieldType {
    fn from(value: f32) -> Self {
        FieldType::F32(value)
    }
}
impl From<f64> for FieldType {
    fn from(value: f64) -> Self {
        FieldType::F64(value)
    }
}
impl From<bool> for FieldType {
    fn from(value: bool) -> Self {
        FieldType::Boolean(value)
    }
}
impl From<String> for FieldType {
    fn from(value: String) -> Self {
        FieldType::String(value)
    }
}
impl From<PrimitiveDateTime> for FieldType {
    fn from(value: PrimitiveDateTime) -> Self {
        FieldType::Time(value)
    }
}
impl From<Uuid> for FieldType {
    fn from(value: Uuid) -> Self {
        FieldType::Uuid(value)
    }
}
impl From<Vec<u8>> for FieldType {
    fn from(value: Vec<u8>) -> Self {
        FieldType::Bytes(value)
    }
}
impl From<JsonValue> for FieldType {
    fn from(value: JsonValue) -> Self {
        FieldType::Json(value)
    }
}
impl From<Vec<String>> for FieldType {
    fn from(value: Vec<String>) -> Self {
        FieldType::Enum(value)
    }
}
impl From<(PostgresColumnType, JsonValue)> for FieldType {
    fn from((sql_type, value): (PostgresColumnType, JsonValue)) -> Self {
        FieldType::Other { sql_type, value }
    }
}
