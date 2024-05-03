use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::{quote};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};

#[derive(std::default::Default, darling::FromMeta)]
enum EntityFieldAttribute {
    #[default]
    None,

    Colname(String),

    Immutable,

    Nullable,

    Unique,
}

struct EntityField {
    field_name: String,
    field_type: String,
    col_name: Option<String>,
    immutable: bool,
    nullable: bool,
    unique: bool,
}

pub fn derive_entity(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    if let syn::Data::Struct(ref s) = input.data { // is this a struct?
        if let syn::Fields::Named(ref fields) = &s.fields { // does this struct have named fields?
            let mut entity_data_fields: Vec<EntityField> = vec![];

            for field in &fields.named {
                let mut entity_field = EntityField {
                    field_name: field
                        .clone()
                        .ident
                        .map_or_else(|| String::from(""), |f| f.to_string()), // TODO: error!
                    field_type: field.ty.to_token_stream().to_string(),
                    col_name: None,
                    immutable: false,
                    nullable: false,
                    unique: false,
                };

                for attr in &field.attrs {
                    match EntityFieldAttribute::from_meta(&attr.meta) {
                        Ok(field_attr) => match field_attr {
                            EntityFieldAttribute::Colname(name) => {
                                entity_field.col_name = Some(name)
                            }
                            EntityFieldAttribute::Immutable => entity_field.immutable = true,
                            EntityFieldAttribute::Nullable => entity_field.nullable = true,
                            EntityFieldAttribute::Unique => entity_field.unique = true,
                            _ => {}
                        },

                        Err(e) => return TokenStream::from(e.write_errors()),
                    }
                }

                entity_data_fields.push(entity_field);
            }

            let field_tokens = entity_data_fields.iter().map(|EntityField { field_name, field_type, col_name, unique, immutable, nullable }| {
                 quote!{
                    graphix::entity::field::Field {
                        name: #field_name.to_string(),
                        column_name: #col_name.to_string(),
                        typ: graphix::entity::field::FieldType::from_str(#field_type).unwrap(),
                        unique: #unique,
                        immutable: #immutable,
                        nullable: #nullable,
                    }
                }
            }).collect::<Vec<proc_macro2::TokenStream>>();

            let tokens = quote! {
                use std::str::FromStr;
                impl graphix::entity::Entity for #name {
                    fn entity_data(&self) -> graphix::entity::EntityData {
                        graphix::entity::EntityData {
                            fields: vec![
                                #(#field_tokens)*,
                            ]
                        }
                    }
                }
            };

            return tokens.into();
        }

        return TokenStream::from(
            syn::Error::new(
                input.span(),
                "Only structs with named fields can derive `Entity`",
            )
            .to_compile_error(),
        )
        .into();
    }

    return TokenStream::from(
        syn::Error::new(input.span(), "Only structs can derive `Entity`").to_compile_error(),
    )
    .into();
}
