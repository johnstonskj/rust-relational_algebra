/*!
One-line description.

More detailed description, with

# Example

 */

use crate::sort::Relation as RelationSchema;
use crate::Identifier;
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataType {
    Byte,
    UnsignedInteger,
    Integer,
    Float,
    Char,
    String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Byte(u8),
    UnsignedInteger(u64),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
}

pub trait Relation {
    fn schema(&self) -> &dyn RelationSchema;

    fn value(&self, index: usize) -> Option<&Value>;

    fn value_by_name(&self, name: &Identifier) -> Option<&Value>;

    fn values(&self) -> Vec<&Value>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Byte => "byte",
                Self::UnsignedInteger => "unsigned",
                Self::Integer => "integer",
                Self::Float => "float",
                Self::Char => "char",
                Self::String => "string",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Byte(v) => format!("0x{:02x}", v),
                Self::UnsignedInteger(v) => format!("{}", v),
                Self::Integer(v) => format!("{}", v),
                Self::Float(v) => format!("{}", v),
                Self::Char(v) => format!("{:?}", v),
                Self::String(v) => format!("{:?}", v),
            }
        )
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Self::Byte(v)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Self::UnsignedInteger(v)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::Integer(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

impl From<char> for Value {
    fn from(v: char) -> Self {
        Self::Char(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl Value {
    #[inline]
    pub fn data_type(&self) -> DataType {
        match self {
            Self::Byte(_) => DataType::Byte,
            Self::UnsignedInteger(_) => DataType::UnsignedInteger,
            Self::Integer(_) => DataType::Integer,
            Self::Float(_) => DataType::Float,
            Self::Char(_) => DataType::Char,
            Self::String(_) => DataType::String,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
