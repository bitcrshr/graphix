use strum::{Display, EnumString};
use crate::dialect::PostgresColumnType;

#[derive(Debug)]
pub struct EntityFieldDescriptor {
    pub name: String,
    pub column_name: String,
    pub typ: String,
    pub sql_type: PostgresColumnType,
    pub unique: bool,
    pub immutable: bool,
    pub nullable: bool,
}
