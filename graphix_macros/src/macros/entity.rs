use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, DeriveInput, Expr, Lit, Meta, MetaNameValue, Token};
use crate::sql::postgres::ColumnType;

enum StructAttribute {
    TableName(String),
    SchemaName(String),
}
impl StructAttribute {
    pub fn from_meta(arg: &MetaNameValue) -> syn::Result<Self> {
        let name = match arg.path.get_ident() {
            Some(i) => i.to_string(),
            None => return Err(syn::Error::new_spanned(arg, "invalid formatting")),
        };

        let value = match &arg.value {
            Expr::Lit(s) => match &s.lit {
                Lit::Str(s) => s.value(),
                _ => return Err(syn::Error::new_spanned(s, "invalid formatting")),
            },
            _ => return Err(syn::Error::new_spanned(&arg.value, "invalid formatting")),
        };

        match name.as_str() {
            "table_name" => {
                if value.is_empty() {
                    return Err(syn::Error::new_spanned(arg, "value cannot be empty"));
                }

                Ok(Self::TableName(value))
            }

            "schema_name" => {
                if value.is_empty() {
                    return Err(syn::Error::new_spanned(arg, "value cannot be empty"));
                }

                Ok(Self::SchemaName(value))
            }

            _ => Err(syn::Error::new_spanned(
                arg,
                format!("unknown attribute `{}`", name),
            )),
        }
    }
}

enum FieldAttribute {
    Unique,
    Immutable,
    Nullable,
    ColumnName(String),
}
impl FieldAttribute {
    pub fn from_meta(meta: &Meta) -> syn::Result<Self> {
        match meta {
            Meta::Path(path) => match path.require_ident()?.to_string().as_str() {
                "immutable" => Ok(Self::Immutable),

                "nullable" => Ok(Self::Nullable),

                "unique" => Ok(Self::Unique),

                _ => Err(syn::Error::new_spanned(
                    path,
                    format!("unknown attribute {}", path.require_ident()?),
                )),
            },

            Meta::NameValue(arg) => {
                let name = match arg.path.get_ident() {
                    Some(i) => i.to_string(),
                    None => return Err(syn::Error::new_spanned(arg, "invalid formatting")),
                };

                let value = match &arg.value {
                    Expr::Lit(s) => match &s.lit {
                        Lit::Str(s) => s.value(),
                        _ => return Err(syn::Error::new_spanned(&arg.value, "invalid formatting")),
                    },
                    _ => return Err(syn::Error::new_spanned(&arg.value, "invalid formatting")),
                };

                match name.as_str() {
                    "colname" => {
                        if value.is_empty() {
                            return Err(syn::Error::new_spanned(
                                &arg.value,
                                "colname attribute must have a value",
                            ));
                        }

                        Ok(Self::ColumnName(value))
                    }

                    _ => Err(syn::Error::new_spanned(
                        arg,
                        format!("unknown attribute {}", name),
                    )),
                }
            }

            _ => Err(syn::Error::new_spanned(meta, "invalid attribute")),
        }
    }
}

pub fn entity_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;

    let fields = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "only structs with named fileds can derive `Entity`",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "only structs can derive `Entity`",
            ))
        }
    };

    // parse struct attrs
    let struct_name = name.to_string();
    let mut table_name = format!("{}s", name.to_string().to_lowercase());
    let mut schema_name = String::from("schema.public");

    for struct_attr in &ast.attrs {
        if !struct_attr.path().is_ident("graphix") {
            continue;
        }

        let args: Punctuated<MetaNameValue, Token![,]> =
            struct_attr.parse_args_with(Punctuated::parse_terminated)?;

        println!("got args: {:?}\n", args);

        for arg in &args {
            match StructAttribute::from_meta(arg) {
                Ok(att) => match att {
                    StructAttribute::TableName(name) => table_name = name,
                    StructAttribute::SchemaName(name) => schema_name = name,
                },

                Err(e) => return Err(e),
            }
        }
    }

    // parse field attrs

    let mut field_desc_tokens: Vec<TokenStream> = Vec::new();

    for field in fields {
        let ident = match &field.ident {
            Some(i) => i.to_string(),
            None => {
                return Err(syn::Error::new_spanned(
                    &field.ident,
                    "struct must have named fields to derive `Entity`",
                ))
            }
        };
        let typ = field.ty.to_token_stream().to_string();
        let sql_typ = ColumnType::try_from(&field.ty)?;
        let mut immutable = false;
        let mut unique = false;
        let mut nullable = false;
        let mut col_name: String = ident.clone();

        for field_attr in &field.attrs {
            if !field_attr.path().is_ident("graphix") {
                continue;
            }

            println!("found attribute: {:#?}\n\n", field_attr);

            let args: Punctuated<syn::Meta, Token![,]> =
                field_attr.parse_args_with(Punctuated::parse_terminated)?;

            for arg in &args {
                match FieldAttribute::from_meta(arg) {
                    Ok(att) => match att {
                        FieldAttribute::Unique => unique = true,
                        FieldAttribute::Immutable => immutable = true,
                        FieldAttribute::Nullable => nullable = true,
                        FieldAttribute::ColumnName(name) => col_name = name,
                    },

                    Err(e) => return Err(e),
                }
            }
        }

        field_desc_tokens.push(quote! {
            graphix::entity::field::EntityFieldDescriptor {
                name: #ident.to_string(),
                typ: #typ.to_string(),
                sql_type: #sql_typ,
                column_name: #col_name.to_string(),
                unique: #unique,
                immutable: #immutable,
                nullable: #nullable,
            }
        });
    }

    println!(
        "table_name: {:?}, schema_name: {:?}\n\nfied descriptors: {:?}\n\n",
        table_name, schema_name, field_desc_tokens
    );

    let output = quote! {
        impl graphix::entity::Entity for #name {
            fn entity_descriptor(&self) -> graphix::entity::EntityDescriptor {
                graphix::entity::EntityDescriptor {
                    name: #struct_name.to_string(),
                    table_name: #table_name.to_string(),
                    schema_name: #schema_name.to_string(),
                    fields: vec![
                        #(#field_desc_tokens)*,
                    ],
                }
            }
        }
    };

    println!("finna output this:\n\n{}", &output.to_string());

    Ok(output)
}

