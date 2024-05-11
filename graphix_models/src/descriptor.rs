use crate::sql::postgres::ColumnType;

#[derive(Debug)]
pub struct EntityDescriptor {
    pub name: String,
    pub table_name: String,
    pub schema_name: String,
    pub fields: Vec<EntityFieldDescriptor>,
}

#[derive(Debug)]
pub struct EntityFieldDescriptor {
    pub name: String,
    pub column_name: String,
    pub typ: String,
    pub sql_type: ColumnType,
    pub unique: bool,
    pub immutable: bool,
    pub nullable: bool,
}
