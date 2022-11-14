/*!
Provides types and traits for describing a *Sort*, more commonly known as a
[`Schema`].

The traits defined here are implemented by a data provider to provide Relation Schema
in some supported store.

 */

use crate::{error::Error, Name};
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A domain is an individual data type. The set of supported domains is referred to
/// as $D$, where $D = \\{D_1, \ldots, D_i\\}$.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Domain {
    Boolean,
    Byte,
    UnsignedInteger,
    Integer,
    Float,
    Char,
    String,
    Binary,
}

///
/// A [`Schema`] $S$ is a named set of [`RelationSchema`] $R$ descriptions; $S = \\{R_1, \ldots, R_i\\}$.
///
pub trait Schema {
    type Item: RelationSchema;

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

///
/// A [`RelationSchema`] $R$ is a named tuple of [`AttributeSchema`] $A$ descriptions; $R = \\{A_1, \ldots, A_i\\}$.
///
#[allow(single_use_lifetimes)]
pub trait RelationSchema {
    type Item: AttributeSchema;

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

///
/// An [`AttributeSchema`] describes one member of a relation tuple; it is a mapping from an attribute name $n$ to a domain; $A = (n \rightarrow d)$ where $d \in D$.
///
pub trait AttributeSchema {
    fn new(name: Name, data_type: Domain) -> Self
    where
        Self: Sized;

    fn name(&self) -> &Name;

    fn domain(&self) -> &Domain;
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

impl Display for Domain {
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
