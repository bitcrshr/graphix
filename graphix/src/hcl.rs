use crate::entity::EntityDescriptor;
use hcl::Block;
use hcl::Body;

pub fn entity_descriptor_to_hcl(desc: &EntityDescriptor) -> Block {
    Block::builder("table").add_label(&desc.name).build()
}
