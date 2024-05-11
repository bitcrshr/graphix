use quote::{quote, ToTokens};
use std::fmt::Formatter;
use syn::spanned::Spanned;
use syn::Type;

pub enum ColumnType {
    Array(Box<ColumnType>),
    Bit(Option<usize>),
    BitVarying(Option<usize>),
    Boolean,
    ByteA,
    Date,
    Time,
    TimeTz,
    Timestamp(Option<usize>),
    TimestampTz,
    Interval,
    Domain(String),
    Enum(String),
    Numeric(Option<Vec<usize>>), // TODO: not sure of arg cardinality
    Real,
    DoublePrecision,
    Float(usize),
    Circle,
    Line,
    LSeg,
    Box,
    Path,
    Polygon,
    Point,
    SmallInt,
    Integer,
    Int,
    BigInt,
    Json,
    Jsonb,
    Money,
    INet,
    Cidr,
    MacAddr,
    MacAddr8,
    Int4Range,
    Int8Range,
    NumRange,
    TsRange,
    TsTzRange,
    DateRange,
    Int4MultiRange,
    Int8MultiRange,
    NumMultiRange,
    TsMultiRange,
    TsTzMultiRange,
    DateMultiRange,
    SmallSerial,
    Serial,
    BigSerial,
    VarChar(Option<usize>),
    Char(Option<usize>),
    Text,
    TsVector,
    TsQuery,
    Uuid,
    Xml,
}
impl std::fmt::Display for ColumnType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(of) => write!(f, "{}[]", **of),
            Self::Bit(maybe_size) => match maybe_size {
                Some(size) => write!(f, "bit({})", size),
                None => write!(f, "bit"),
            },
            Self::BitVarying(maybe_size) => match maybe_size {
                Some(size) => write!(f, "bit_varying({})", size),
                None => write!(f, "bit_varying"),
            },
            Self::Boolean => write!(f, "boolean"),
            Self::ByteA => write!(f, "bytea"),
            Self::Date => write!(f, "date"),
            Self::Time => write!(f, "time"),
            Self::TimeTz => write!(f, "timetz"),
            Self::Timestamp(maybe_precision) => match maybe_precision {
                Some(precision) => write!(f, "timestamp({})", precision),
                None => write!(f, "timestamp"),
            },
            Self::TimestampTz => write!(f, "timestamptz"),
            Self::Interval => write!(f, "interval"),
            Self::Domain(name) => write!(f, "domain.{}", name),
            Self::Enum(name) => write!(f, "enum.{}", name),
            Self::Numeric(maybe_args) => match maybe_args {
                Some(args) => write!(
                    f,
                    "numeric({})",
                    args.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                ),
                None => write!(f, "numeric"),
            },
            Self::Real => write!(f, "real"),
            Self::DoublePrecision => write!(f, "double_precision"),
            Self::Float(precision) => write!(f, "float({})", precision),
            Self::Circle => write!(f, "circle"),
            Self::Line => write!(f, "line"),
            Self::LSeg => write!(f, "lseg"),
            Self::Box => write!(f, "box"),
            Self::Path => write!(f, "path"),
            Self::Polygon => write!(f, "polygon"),
            Self::Point => write!(f, "point"),
            Self::SmallInt => write!(f, "smallint"),
            Self::Integer => write!(f, "integer"),
            Self::Int => write!(f, "int"),
            Self::BigInt => write!(f, "bigint"),
            Self::Json => write!(f, "json"),
            Self::Jsonb => write!(f, "jsonb"),
            Self::Money => write!(f, "money"),
            Self::INet => write!(f, "inet"),
            Self::Cidr => write!(f, "cidr"),
            Self::MacAddr => write!(f, "macaddr"),
            Self::MacAddr8 => write!(f, "macaddr8"),
            Self::Int4Range => write!(f, "int4range"),
            Self::Int8Range => write!(f, "int8range"),
            Self::NumRange => write!(f, "numrange"),
            Self::TsRange => write!(f, "tsrange"),
            Self::TsTzRange => write!(f, "tstzrange"),
            Self::DateRange => write!(f, "daterange"),
            Self::Int4MultiRange => write!(f, "int4multirange"),
            Self::Int8MultiRange => write!(f, "int8multirange"),
            Self::NumMultiRange => write!(f, "nummultirange"),
            Self::TsMultiRange => write!(f, "tsmultirange"),
            Self::TsTzMultiRange => write!(f, "tstzmultirange"),
            Self::DateMultiRange => write!(f, "datemultirange"),
            Self::SmallSerial => write!(f, "smallserial"),
            Self::Serial => write!(f, "serial"),
            Self::BigSerial => write!(f, "bigserial"),
            Self::VarChar(maybe_length) => match maybe_length {
                Some(length) => write!(f, "varchar({})", length),
                None => write!(f, "varchar"),
            },
            Self::Char(maybe_length) => match maybe_length {
                Some(length) => write!(f, "char({})", length),
                None => write!(f, "char"),
            },
            Self::Text => write!(f, "text"),
            Self::TsVector => write!(f, "tsvector"),
            Self::TsQuery => write!(f, "tsquery"),
            Self::Uuid => write!(f, "uuid"),
            Self::Xml => write!(f, "xml"),
        }
    }
}
impl TryFrom<&syn::Type> for ColumnType {
    type Error = syn::Error;

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        let type_string = value.to_token_stream().to_string();

        match value {
            syn::Type::Path(tp) => {
                let ident = tp.path.require_ident()?;

                match ident.to_string().as_str() {
                    "String" => Ok(Self::Text),
                    "i8" => Ok(Self::SmallInt),
                    "i16" => Ok(Self::SmallInt),
                    "i32" => Ok(Self::Integer),
                    "i64" => Ok(Self::BigInt),
                    "isize" => Ok(Self::BigInt),
                    "u8" => Ok(Self::SmallInt),
                    "u16" => Ok(Self::SmallInt),
                    "u32" => Ok(Self::Integer),
                    "u64" => Ok(Self::BigInt),
                    "usize" => Ok(Self::BigInt),
                    "f32" => Ok(Self::Real),
                    "f64" => Ok(Self::DoublePrecision),
                    "bool" => Ok(Self::Boolean),
                    "char" => Ok(Self::Char(Some(1))),
                    // TODO: handle compound types
                    // TODO: handle nonstd types
                    _ => Err(syn::Error::new_spanned(
                        ident,
                        format!(
                            "graphix is currently unable to handle type: {}",
                            type_string,
                        ),
                    )),
                }
            }

            _ => Err(syn::Error::new_spanned(value, "invalid type")),
        }
    }
}
impl ToTokens for ColumnType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      tokens.extend(match self {
          Self::Text => quote! { graphix::dialect::PostgresColumnType::Text },
            Self::SmallInt => quote! { graphix::dialect::PostgresColumnType::SmallInt },
            Self::Integer => quote! { graphix::dialect::PostgresColumnType::Integer },
            Self::BigInt => quote! { graphix::dialect::PostgresColumnType::BigInt },
            Self::Real => quote! { graphix::dialect::PostgresColumnType::Real },
            Self::DoublePrecision => quote! { graphix::dialect::PostgresColumnType::DoublePrecision },
            Self::Boolean => quote! { graphix::dialect::PostgresColumnType::Boolean },
           _ => syn::Error::new(tokens.span(), format!("graphix does not yet support the column type {}", self)).to_compile_error()
       });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array() {
        assert_eq!(
            ColumnType::Array(Box::new(ColumnType::Integer)).to_string(),
            "integer[]"
        );
    }

    #[test]
    fn test_bit() {
        assert_eq!(ColumnType::Bit(None).to_string(), "bit");
        assert_eq!(ColumnType::Bit(Some(5)).to_string(), "bit(5)");
    }

    #[test]
    fn test_bit_varying() {
        assert_eq!(ColumnType::BitVarying(None).to_string(), "bit_varying");
        assert_eq!(
            ColumnType::BitVarying(Some(5)).to_string(),
            "bit_varying(5)"
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(ColumnType::Boolean.to_string(), "boolean");
    }

    #[test]
    fn test_bytea() {
        assert_eq!(ColumnType::ByteA.to_string(), "bytea");
    }

    #[test]
    fn test_date() {
        assert_eq!(ColumnType::Date.to_string(), "date");
    }

    #[test]
    fn test_time() {
        assert_eq!(ColumnType::Time.to_string(), "time");
    }

    #[test]
    fn test_time_tz() {
        assert_eq!(ColumnType::TimeTz.to_string(), "timetz");
    }

    #[test]
    fn test_timestamp() {
        assert_eq!(ColumnType::Timestamp(None).to_string(), "timestamp");
        assert_eq!(ColumnType::Timestamp(Some(5)).to_string(), "timestamp(5)");
    }

    #[test]
    fn test_timestamp_tz() {
        assert_eq!(ColumnType::TimestampTz.to_string(), "timestamptz");
    }

    #[test]
    fn test_interval() {
        assert_eq!(ColumnType::Interval.to_string(), "interval");
    }

    #[test]
    fn test_domain() {
        assert_eq!(
            ColumnType::Domain("foo".to_string()).to_string(),
            "domain.foo"
        );
    }

    #[test]
    fn test_enum() {
        assert_eq!(ColumnType::Enum("foo".to_string()).to_string(), "enum.foo");
    }

    #[test]
    fn test_numeric() {
        assert_eq!(ColumnType::Numeric(None).to_string(), "numeric");
        assert_eq!(
            ColumnType::Numeric(Some(vec![1, 2, 3])).to_string(),
            "numeric(1,2,3)"
        );
    }

    #[test]
    fn test_real() {
        assert_eq!(ColumnType::Real.to_string(), "real");
    }

    #[test]
    fn test_double_precision() {
        assert_eq!(ColumnType::DoublePrecision.to_string(), "double_precision");
    }

    #[test]
    fn test_float() {
        assert_eq!(ColumnType::Float(5).to_string(), "float(5)");
    }

    #[test]
    fn test_circle() {
        assert_eq!(ColumnType::Circle.to_string(), "circle");
    }

    #[test]
    fn test_line() {
        assert_eq!(ColumnType::Line.to_string(), "line");
    }

    #[test]
    fn test_lseg() {
        assert_eq!(ColumnType::LSeg.to_string(), "lseg");
    }

    #[test]
    fn test_box() {
        assert_eq!(ColumnType::Box.to_string(), "box");
    }

    #[test]
    fn test_path() {
        assert_eq!(ColumnType::Path.to_string(), "path");
    }

    #[test]
    fn test_polygon() {
        assert_eq!(ColumnType::Polygon.to_string(), "polygon");
    }

    #[test]
    fn test_point() {
        assert_eq!(ColumnType::Point.to_string(), "point");
    }

    #[test]
    fn test_smallint() {
        assert_eq!(ColumnType::SmallInt.to_string(), "smallint");
    }

    #[test]
    fn test_integer() {
        assert_eq!(ColumnType::Integer.to_string(), "integer");
    }

    #[test]
    fn test_int() {
        assert_eq!(ColumnType::Int.to_string(), "int");
    }

    #[test]
    fn test_bigint() {
        assert_eq!(ColumnType::BigInt.to_string(), "bigint");
    }

    #[test]
    fn test_json() {
        assert_eq!(ColumnType::Json.to_string(), "json");
    }

    #[test]
    fn test_jsonb() {
        assert_eq!(ColumnType::Jsonb.to_string(), "jsonb");
    }

    #[test]
    fn test_money() {
        assert_eq!(ColumnType::Money.to_string(), "money");
    }

    #[test]
    fn test_inet() {
        assert_eq!(ColumnType::INet.to_string(), "inet");
    }

    #[test]
    fn test_cidr() {
        assert_eq!(ColumnType::Cidr.to_string(), "cidr");
    }

    #[test]
    fn test_mac_addr() {
        assert_eq!(ColumnType::MacAddr.to_string(), "macaddr");
    }

    #[test]
    fn test_mac_addr8() {
        assert_eq!(ColumnType::MacAddr8.to_string(), "macaddr8");
    }

    #[test]
    fn test_int4_range() {
        assert_eq!(ColumnType::Int4Range.to_string(), "int4range");
    }

    #[test]
    fn test_int8_range() {
        assert_eq!(ColumnType::Int8Range.to_string(), "int8range");
    }

    #[test]
    fn test_num_range() {
        assert_eq!(ColumnType::NumRange.to_string(), "numrange");
    }

    #[test]
    fn test_ts_range() {
        assert_eq!(ColumnType::TsRange.to_string(), "tsrange");
    }

    #[test]
    fn test_ts_tz_range() {
        assert_eq!(ColumnType::TsTzRange.to_string(), "tstzrange");
    }

    #[test]
    fn test_date_range() {
        assert_eq!(ColumnType::DateRange.to_string(), "daterange");
    }

    #[test]
    fn test_int4_multi_range() {
        assert_eq!(ColumnType::Int4MultiRange.to_string(), "int4multirange");
    }

    #[test]
    fn test_int8_multi_range() {
        assert_eq!(ColumnType::Int8MultiRange.to_string(), "int8multirange");
    }

    #[test]
    fn test_num_multi_range() {
        assert_eq!(ColumnType::NumMultiRange.to_string(), "nummultirange");
    }

    #[test]
    fn test_ts_multi_range() {
        assert_eq!(ColumnType::TsMultiRange.to_string(), "tsmultirange");
    }

    #[test]
    fn test_ts_tz_multi_range() {
        assert_eq!(ColumnType::TsTzMultiRange.to_string(), "tstzmultirange");
    }

    #[test]
    fn test_date_multi_range() {
        assert_eq!(ColumnType::DateMultiRange.to_string(), "datemultirange");
    }

    #[test]
    fn test_small_serial() {
        assert_eq!(ColumnType::SmallSerial.to_string(), "smallserial");
    }

    #[test]
    fn test_serial() {
        assert_eq!(ColumnType::Serial.to_string(), "serial");
    }

    #[test]
    fn test_big_serial() {
        assert_eq!(ColumnType::BigSerial.to_string(), "bigserial");
    }

    #[test]
    fn test_varchar() {
        assert_eq!(ColumnType::VarChar(None).to_string(), "varchar");
        assert_eq!(ColumnType::VarChar(Some(5)).to_string(), "varchar(5)");
    }

    #[test]
    fn test_char() {
        assert_eq!(ColumnType::Char(None).to_string(), "char");
        assert_eq!(ColumnType::Char(Some(5)).to_string(), "char(5)");
    }

    #[test]
    fn test_text() {
        assert_eq!(ColumnType::Text.to_string(), "text");
    }

    #[test]
    fn test_ts_vector() {
        assert_eq!(ColumnType::TsVector.to_string(), "tsvector");
    }

    #[test]
    fn test_ts_query() {
        assert_eq!(ColumnType::TsQuery.to_string(), "tsquery");
    }

    #[test]
    fn test_uuid() {
        assert_eq!(ColumnType::Uuid.to_string(), "uuid");
    }

    #[test]
    fn test_xml() {
        assert_eq!(ColumnType::Xml.to_string(), "xml");
    }
}
