use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString)]
pub enum FieldType {
    String,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub column_name: String,
    pub typ: FieldType,
    pub unique: bool,
    pub immutable: bool,
    pub nullable: bool,
}