use crate::descriptor::{EntityDescriptor, EntityFieldDescriptor};
use hcl::Block;

pub trait Entity {
    fn entity_descriptor(&self) -> EntityDescriptor;
    fn as_atlas_hcl(&self) -> Block {
        let desc = self.entity_descriptor();

        let mut builder = Block::builder("table")
            .add_label(desc.table_name)
            .add_attribute(("schema", desc.schema_name));

        builder.build()
    }
}
