use hcl::Block;

mod attribute;
pub mod field;
mod field_type;

#[derive(Debug)]
pub struct EntityDescriptor {
    pub name: String,
    pub table_name: String,
    pub schema_name: String,
    pub fields: Vec<field::EntityFieldDescriptor>,
}

pub trait Entity {
    fn entity_descriptor(&self) -> EntityDescriptor;
    fn as_atlas_hcl(&self) -> hcl::Block {
        let desc = self.entity_descriptor();

        let mut builder = Block::builder("table")
            .add_label(desc.table_name)
            .add_attribute(("schema", desc.schema_name));

        builder.build()
    }
}
