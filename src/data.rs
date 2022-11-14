/*!
Provides the traits required to implement a Relation *instance* and Tuple *instance* for evaluation.
 */

use crate::sort::{Domain, RelationSchema};
use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Boolean(bool),
    Byte(u8),
    UnsignedInteger(u64),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Binary(Vec<u8>),
}

///
/// A [`Relation`] $r$ is a set of tuples, and conforms to a specific [`RelationSchema`].
///
pub trait Relation {
    type Schema: RelationSchema;
    type Item: Tuple;

    fn schema(&self) -> &Self::Schema;

    fn tuples(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_>;
}

///
/// A [`Tuple`], or *relation instance*, $t$ comprises attribute $a$ [`Value`]s, conforming to specific `AttributeSchema`s.
///
#[allow(single_use_lifetimes)]
pub trait Tuple {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn value(&self, index: usize) -> Option<&Value>;

    fn values(&self) -> Box<dyn Iterator<Item = &Value> + '_>;
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean(v) => format!("{}", v),
                Self::Byte(v) => format!("0x{:02x}", v),
                Self::UnsignedInteger(v) => format!("{}", v),
                Self::Integer(v) => format!("{}", v),
                Self::Float(v) => format!("{}", v),
                Self::Char(v) => format!("{:?}", v),
                Self::String(v) => format!("{:?}", v),
                Self::Binary(v) => format!("{:?}", v),
            }
        )
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
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

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Self::Binary(v)
    }
}

impl Value {
    #[inline]
    pub fn data_type(&self) -> Domain {
        match self {
            Self::Boolean(_) => Domain::Boolean,
            Self::Byte(_) => Domain::Byte,
            Self::UnsignedInteger(_) => Domain::UnsignedInteger,
            Self::Integer(_) => Domain::Integer,
            Self::Float(_) => Domain::Float,
            Self::Char(_) => Domain::Char,
            Self::String(_) => Domain::String,
            Self::Binary(_) => Domain::Binary,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
