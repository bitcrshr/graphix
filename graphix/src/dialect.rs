use std::fmt::Formatter;

pub enum PostgresColumnType {
    Array(Box<PostgresColumnType>),
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
impl std::fmt::Display for PostgresColumnType {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array() {
        assert_eq!(
            PostgresColumnType::Array(Box::new(PostgresColumnType::Integer)).to_string(),
            "integer[]"
        );
    }

    #[test]
    fn test_bit() {
        assert_eq!(PostgresColumnType::Bit(None).to_string(), "bit");
        assert_eq!(PostgresColumnType::Bit(Some(5)).to_string(), "bit(5)");
    }

    #[test]
    fn test_bit_varying() {
        assert_eq!(
            PostgresColumnType::BitVarying(None).to_string(),
            "bit_varying"
        );
        assert_eq!(
            PostgresColumnType::BitVarying(Some(5)).to_string(),
            "bit_varying(5)"
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(PostgresColumnType::Boolean.to_string(), "boolean");
    }

    #[test]
    fn test_bytea() {
        assert_eq!(PostgresColumnType::ByteA.to_string(), "bytea");
    }

    #[test]
    fn test_date() {
        assert_eq!(PostgresColumnType::Date.to_string(), "date");
    }

    #[test]
    fn test_time() {
        assert_eq!(PostgresColumnType::Time.to_string(), "time");
    }

    #[test]
    fn test_time_tz() {
        assert_eq!(PostgresColumnType::TimeTz.to_string(), "timetz");
    }

    #[test]
    fn test_timestamp() {
        assert_eq!(PostgresColumnType::Timestamp(None).to_string(), "timestamp");
        assert_eq!(
            PostgresColumnType::Timestamp(Some(5)).to_string(),
            "timestamp(5)"
        );
    }

    #[test]
    fn test_timestamp_tz() {
        assert_eq!(PostgresColumnType::TimestampTz.to_string(), "timestamptz");
    }

    #[test]
    fn test_interval() {
        assert_eq!(PostgresColumnType::Interval.to_string(), "interval");
    }

    #[test]
    fn test_domain() {
        assert_eq!(
            PostgresColumnType::Domain("foo".to_string()).to_string(),
            "domain.foo"
        );
    }

    #[test]
    fn test_enum() {
        assert_eq!(
            PostgresColumnType::Enum("foo".to_string()).to_string(),
            "enum.foo"
        );
    }

    #[test]
    fn test_numeric() {
        assert_eq!(PostgresColumnType::Numeric(None).to_string(), "numeric");
        assert_eq!(
            PostgresColumnType::Numeric(Some(vec![1, 2, 3])).to_string(),
            "numeric(1,2,3)"
        );
    }

    #[test]
    fn test_real() {
        assert_eq!(PostgresColumnType::Real.to_string(), "real");
    }

    #[test]
    fn test_double_precision() {
        assert_eq!(
            PostgresColumnType::DoublePrecision.to_string(),
            "double_precision"
        );
    }

    #[test]
    fn test_float() {
        assert_eq!(PostgresColumnType::Float(5).to_string(), "float(5)");
    }

    #[test]
    fn test_circle() {
        assert_eq!(PostgresColumnType::Circle.to_string(), "circle");
    }

    #[test]
    fn test_line() {
        assert_eq!(PostgresColumnType::Line.to_string(), "line");
    }

    #[test]
    fn test_lseg() {
        assert_eq!(PostgresColumnType::LSeg.to_string(), "lseg");
    }

    #[test]
    fn test_box() {
        assert_eq!(PostgresColumnType::Box.to_string(), "box");
    }

    #[test]
    fn test_path() {
        assert_eq!(PostgresColumnType::Path.to_string(), "path");
    }

    #[test]
    fn test_polygon() {
        assert_eq!(PostgresColumnType::Polygon.to_string(), "polygon");
    }

    #[test]
    fn test_point() {
        assert_eq!(PostgresColumnType::Point.to_string(), "point");
    }

    #[test]
    fn test_smallint() {
        assert_eq!(PostgresColumnType::SmallInt.to_string(), "smallint");
    }

    #[test]
    fn test_integer() {
        assert_eq!(PostgresColumnType::Integer.to_string(), "integer");
    }

    #[test]
    fn test_int() {
        assert_eq!(PostgresColumnType::Int.to_string(), "int");
    }

    #[test]
    fn test_bigint() {
        assert_eq!(PostgresColumnType::BigInt.to_string(), "bigint");
    }

    #[test]
    fn test_json() {
        assert_eq!(PostgresColumnType::Json.to_string(), "json");
    }

    #[test]
    fn test_jsonb() {
        assert_eq!(PostgresColumnType::Jsonb.to_string(), "jsonb");
    }

    #[test]
    fn test_money() {
        assert_eq!(PostgresColumnType::Money.to_string(), "money");
    }

    #[test]
    fn test_inet() {
        assert_eq!(PostgresColumnType::INet.to_string(), "inet");
    }

    #[test]
    fn test_cidr() {
        assert_eq!(PostgresColumnType::Cidr.to_string(), "cidr");
    }

    #[test]
    fn test_mac_addr() {
        assert_eq!(PostgresColumnType::MacAddr.to_string(), "macaddr");
    }

    #[test]
    fn test_mac_addr8() {
        assert_eq!(PostgresColumnType::MacAddr8.to_string(), "macaddr8");
    }

    #[test]
    fn test_int4_range() {
        assert_eq!(PostgresColumnType::Int4Range.to_string(), "int4range");
    }

    #[test]
    fn test_int8_range() {
        assert_eq!(PostgresColumnType::Int8Range.to_string(), "int8range");
    }

    #[test]
    fn test_num_range() {
        assert_eq!(PostgresColumnType::NumRange.to_string(), "numrange");
    }

    #[test]
    fn test_ts_range() {
        assert_eq!(PostgresColumnType::TsRange.to_string(), "tsrange");
    }

    #[test]
    fn test_ts_tz_range() {
        assert_eq!(PostgresColumnType::TsTzRange.to_string(), "tstzrange");
    }

    #[test]
    fn test_date_range() {
        assert_eq!(PostgresColumnType::DateRange.to_string(), "daterange");
    }

    #[test]
    fn test_int4_multi_range() {
        assert_eq!(
            PostgresColumnType::Int4MultiRange.to_string(),
            "int4multirange"
        );
    }

    #[test]
    fn test_int8_multi_range() {
        assert_eq!(
            PostgresColumnType::Int8MultiRange.to_string(),
            "int8multirange"
        );
    }

    #[test]
    fn test_num_multi_range() {
        assert_eq!(
            PostgresColumnType::NumMultiRange.to_string(),
            "nummultirange"
        );
    }

    #[test]
    fn test_ts_multi_range() {
        assert_eq!(PostgresColumnType::TsMultiRange.to_string(), "tsmultirange");
    }

    #[test]
    fn test_ts_tz_multi_range() {
        assert_eq!(
            PostgresColumnType::TsTzMultiRange.to_string(),
            "tstzmultirange"
        );
    }

    #[test]
    fn test_date_multi_range() {
        assert_eq!(
            PostgresColumnType::DateMultiRange.to_string(),
            "datemultirange"
        );
    }

    #[test]
    fn test_small_serial() {
        assert_eq!(PostgresColumnType::SmallSerial.to_string(), "smallserial");
    }

    #[test]
    fn test_serial() {
        assert_eq!(PostgresColumnType::Serial.to_string(), "serial");
    }

    #[test]
    fn test_big_serial() {
        assert_eq!(PostgresColumnType::BigSerial.to_string(), "bigserial");
    }

    #[test]
    fn test_varchar() {
        assert_eq!(PostgresColumnType::VarChar(None).to_string(), "varchar");
        assert_eq!(
            PostgresColumnType::VarChar(Some(5)).to_string(),
            "varchar(5)"
        );
    }

    #[test]
    fn test_char() {
        assert_eq!(PostgresColumnType::Char(None).to_string(), "char");
        assert_eq!(PostgresColumnType::Char(Some(5)).to_string(), "char(5)");
    }

    #[test]
    fn test_text() {
        assert_eq!(PostgresColumnType::Text.to_string(), "text");
    }

    #[test]
    fn test_ts_vector() {
        assert_eq!(PostgresColumnType::TsVector.to_string(), "tsvector");
    }

    #[test]
    fn test_ts_query() {
        assert_eq!(PostgresColumnType::TsQuery.to_string(), "tsquery");
    }

    #[test]
    fn test_uuid() {
        assert_eq!(PostgresColumnType::Uuid.to_string(), "uuid");
    }

    #[test]
    fn test_xml() {
        assert_eq!(PostgresColumnType::Xml.to_string(), "xml");
    }
}
