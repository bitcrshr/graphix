use strum::{Display, EnumString};

#[derive(Debug)]
pub struct EntityFieldDescriptor {
    pub name: String,
    pub column_name: String,
    pub typ: String,
    pub unique: bool,
    pub immutable: bool,
    pub nullable: bool,
}
