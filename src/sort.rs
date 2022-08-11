/*!
One-line description.

More detailed description, with

# Example

 */

use crate::data::DataType;
use crate::Identifier;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Sort: Display {
    fn new(name: Identifier, relations: Vec<Box<dyn Relation>>) -> Self
    where
        Self: Sized;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn name(&self) -> &Identifier;

    fn has_relation(&self, name: &Identifier) -> bool;

    fn relation(&self, name: &Identifier) -> Option<Box<dyn Relation>>;

    fn relations(&self) -> Vec<&dyn Relation>;
}

pub trait Relation: Display {
    fn new(name: Identifier, attributes: Vec<Box<dyn Attribute>>) -> Self
    where
        Self: Sized;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn name(&self) -> &Identifier;

    fn has_attribute(&self, name: &Identifier) -> bool {
        self.attribute_index(name).is_some()
    }

    fn attribute_index(&self, name: &Identifier) -> Option<usize>;

    fn attribute(&self, name: &Identifier) -> Option<Box<dyn Attribute>>;

    fn attributes(&self) -> Vec<&dyn Attribute>;
}

pub trait Attribute: Display {
    fn new(name: Identifier, data_type: DataType) -> Self
    where
        Self: Sized;

    fn name(&self) -> &Identifier;

    fn rename(&self, name: Identifier) -> Self
    where
        Self: Sized;

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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
