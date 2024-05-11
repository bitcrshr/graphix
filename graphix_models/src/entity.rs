use crate::descriptor::{EntityDescriptor, EntityFieldDescriptor};
use hcl::expr::TraversalBuilder;
use hcl::{Block, Expression, Identifier, Object, Traversal, TraversalOperator, Variable};

pub trait Entity {
    fn entity_descriptor(&self) -> EntityDescriptor;
    fn as_atlas_hcl(&self) -> Block {
        let desc = &self.entity_descriptor();

        let mut builder = Block::builder("table")
            .add_label(&desc.table_name)
            .add_attribute(("schema", desc.schema_name.clone()));

        for field in &desc.fields {
            builder = builder.add_block(
                Block::builder("column")
                    .add_label(field.column_name.clone())
                    .add_attribute(("type", field.sql_type.to_string()))
                    .add_attribute(("null", field.nullable))
                    .build(),
            );

            if field.unique {
                let table_name = desc.table_name.clone();
                let colname = field.column_name.clone();
                builder = builder.add_block(
                    Block::builder("index")
                        .add_label(format!("idx_{}_{}_unique", table_name, field.column_name.clone()))
                        .add_attribute(("unique", true))
                        .add_attribute((
                            "columns",
                            vec![Expression::Traversal(Box::new(Traversal::new(
                                Variable::new("column").expect("failed to create variable `column`"),
                                vec![TraversalOperator::GetAttr(Identifier::from(colname))],
                            )))],
                        ))
                        .build(),
                );
            }
        }

        builder.build()
    }
}
