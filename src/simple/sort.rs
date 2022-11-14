/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{
    sort::{AttributeSchema, Domain, RelationSchema, Schema},
    Name,
};
use std::{collections::HashMap, fmt::Display};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct SimpleSchema {
    name: Name,
    relations: HashMap<Name, SimpleRelationSchema>,
}

#[derive(Debug)]
pub struct Relations<'a> {
    iter: std::collections::hash_map::Values<'a, Name, SimpleRelationSchema>,
}

#[derive(Clone, Debug)]
pub struct SimpleRelationSchema {
    name: Name,
    attributes: Vec<SimpleAttributeSchema>,
}

#[derive(Debug)]
pub struct Attributes<'a> {
    iter: std::slice::Iter<'a, SimpleAttributeSchema>,
}

#[derive(Clone, Debug)]
pub struct SimpleAttributeSchema {
    name: Name,
    data_type: Domain,
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

impl Display for SimpleSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{{{}}}",
            self.name(),
            self.relations()
                .map(|r| r.name().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Schema for SimpleSchema {
    type Item = SimpleRelationSchema;

    fn new<I>(name: Name, relations: I) -> Result<Self, crate::error::Error>
    where
        I: IntoIterator<Item = Self::Item>,
        Self: Sized,
    {
        Ok(Self {
            name,
            relations: HashMap::from_iter(relations.into_iter().map(|r| (r.name().clone(), r))),
        })
    }

    fn len(&self) -> usize {
        self.relations.len()
    }

    fn name(&self) -> &Name {
        &self.name
    }

    fn relation(&self, name: &Name) -> Option<&Self::Item> {
        self.relations.get(name)
    }

    fn relations(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(Relations {
            iter: self.relations.values(),
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for Relations<'a> {
    type Item = &'a SimpleRelationSchema;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SimpleRelationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.name(),
            self.attributes()
                .map(SimpleAttributeSchema::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl RelationSchema for SimpleRelationSchema {
    type Item = SimpleAttributeSchema;

    fn new<I>(name: Name, attributes: I) -> Result<Self, crate::error::Error>
    where
        I: IntoIterator<Item = Self::Item>,
        Self: Sized,
    {
        Ok(Self {
            name,
            attributes: Vec::from_iter(attributes),
        })
    }

    fn len(&self) -> usize {
        self.attributes.len()
    }

    fn name(&self) -> &Name {
        &self.name
    }

    fn attribute(&self, index: usize) -> Option<&Self::Item> {
        self.attributes.get(index)
    }

    fn attributes(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(Attributes {
            iter: self.attributes.iter(),
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for Attributes<'a> {
    type Item = &'a SimpleAttributeSchema;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SimpleAttributeSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name(), self.domain())
    }
}

impl AttributeSchema for SimpleAttributeSchema {
    fn new(name: Name, data_type: Domain) -> Self
    where
        Self: Sized,
    {
        Self { name, data_type }
    }

    fn name(&self) -> &Name {
        &self.name
    }

    fn domain(&self) -> &Domain {
        &self.data_type
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
