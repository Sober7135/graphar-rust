use std::fmt;

/// GraphAr logical data type used by property metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DataType {
    /// Boolean value.
    Bool,
    /// Signed 8-bit integer.
    Int8,
    /// Signed 16-bit integer.
    Int16,
    /// Signed 32-bit integer.
    Int32,
    /// Signed 64-bit integer.
    Int64,
    /// Unsigned 8-bit integer.
    UInt8,
    /// Unsigned 16-bit integer.
    UInt16,
    /// Unsigned 32-bit integer.
    UInt32,
    /// Unsigned 64-bit integer.
    UInt64,
    /// 32-bit floating-point number.
    Float32,
    /// 64-bit floating-point number.
    Float64,
    /// UTF-8 string.
    String,
    /// Date with day unit.
    Date32,
    /// Timestamp with millisecond unit.
    TimestampMillis,
    /// List of another data type.
    List(Box<DataType>),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool => f.write_str("bool"),
            Self::Int8 => f.write_str("int8"),
            Self::Int16 => f.write_str("int16"),
            Self::Int32 => f.write_str("int32"),
            Self::Int64 => f.write_str("int64"),
            Self::UInt8 => f.write_str("uint8"),
            Self::UInt16 => f.write_str("uint16"),
            Self::UInt32 => f.write_str("uint32"),
            Self::UInt64 => f.write_str("uint64"),
            Self::Float32 => f.write_str("float"),
            Self::Float64 => f.write_str("double"),
            Self::String => f.write_str("string"),
            Self::Date32 => f.write_str("date"),
            Self::TimestampMillis => f.write_str("timestamp"),
            Self::List(inner) => write!(f, "list<{inner}>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataType;

    #[test]
    fn display_common_types() {
        let cases = [
            (DataType::Bool, "bool"),
            (DataType::Int8, "int8"),
            (DataType::Int16, "int16"),
            (DataType::Int32, "int32"),
            (DataType::Int64, "int64"),
            (DataType::UInt8, "uint8"),
            (DataType::UInt16, "uint16"),
            (DataType::UInt32, "uint32"),
            (DataType::UInt64, "uint64"),
            (DataType::Float32, "float"),
            (DataType::Float64, "double"),
            (DataType::String, "string"),
            (DataType::Date32, "date"),
            (DataType::TimestampMillis, "timestamp"),
        ];
        for (value, output) in cases {
            assert_eq!(value.to_string(), output);
        }
    }

    #[test]
    fn display_list_type() {
        let value = DataType::List(Box::new(DataType::UInt32));
        assert_eq!(value.to_string(), "list<uint32>");
    }
}
