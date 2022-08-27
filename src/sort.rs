/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{error::Error, Name};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataType {
    Boolean,
    Byte,
    UnsignedInteger,
    Integer,
    Float,
    Char,
    String,
    Binary,
}

pub trait Sort: Display {
    type Item: SortRelation;

    fn new<I>(name: Name, relations: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = Self::Item>,
        Self: Sized;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn name(&self) -> &Name;

    fn has_relation(&self, name: &Name) -> bool {
        self.relation(name).is_some()
    }

    fn relation(&self, name: &Name) -> Option<&Self::Item>;

    fn relations(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_>;
}

#[allow(single_use_lifetimes)]
pub trait SortRelation: Display {
    type Item: SortAttribute;

    fn new<I>(name: Name, attributes: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = Self::Item>,
        Self: Sized;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn name(&self) -> &Name;

    fn has_attribute(&self, name: &Name) -> bool {
        self.attribute_index(name).is_some()
    }

    fn attribute_index(&self, name: &Name) -> Option<usize> {
        self.attributes()
            .enumerate()
            .find_map(|(i, a)| if a.name() == name { Some(i) } else { None })
    }

    fn attribute(&self, index: usize) -> Option<&Self::Item>;

    fn attributes(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_>;
}

pub trait SortAttribute: Display {
    fn new(name: Name, data_type: DataType) -> Self
    where
        Self: Sized;

    fn name(&self) -> &Name;

    fn data_type(&self) -> &DataType;
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
                Self::Boolean => "boolean",
                Self::Byte => "byte",
                Self::UnsignedInteger => "unsigned",
                Self::Integer => "integer",
                Self::Float => "float",
                Self::Char => "char",
                Self::String => "string",
                Self::Binary => "binary",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
