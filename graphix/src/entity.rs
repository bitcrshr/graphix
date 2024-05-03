pub mod field;

#[derive(Debug)]
pub struct EntityData {
    pub fields: Vec<field::Field>,
}

pub trait Entity {
    fn entity_data(&self) -> EntityData;
}
