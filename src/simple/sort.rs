/*!
One-line description.

More detailed description, with

# Example

 */

use crate::{
    sort::{DataType, Sort, SortAttribute, SortRelation},
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
pub struct SimpleSort {
    name: Name,
    relations: HashMap<Name, SimpleSortRelation>,
}

#[derive(Debug)]
pub struct Relations<'a> {
    iter: std::collections::hash_map::Values<'a, Name, SimpleSortRelation>,
}

#[derive(Clone, Debug)]
pub struct SimpleSortRelation {
    name: Name,
    attributes: Vec<SimpleSortAttribute>,
}

#[derive(Debug)]
pub struct Attributes<'a> {
    iter: std::slice::Iter<'a, SimpleSortAttribute>,
}

#[derive(Clone, Debug)]
pub struct SimpleSortAttribute {
    name: Name,
    data_type: DataType,
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

impl Display for SimpleSort {
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

impl Sort for SimpleSort {
    type Item = SimpleSortRelation;

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
    type Item = &'a SimpleSortRelation;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SimpleSortRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({})",
            self.name(),
            self.attributes()
                .map(SimpleSortAttribute::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl SortRelation for SimpleSortRelation {
    type Item = SimpleSortAttribute;

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
    type Item = &'a SimpleSortAttribute;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SimpleSortAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name(), self.data_type())
    }
}

impl SortAttribute for SimpleSortAttribute {
    fn new(name: Name, data_type: DataType) -> Self
    where
        Self: Sized,
    {
        Self { name, data_type }
    }

    fn name(&self) -> &Name {
        &&self.name
    }

    fn data_type(&self) -> &DataType {
        &&self.data_type
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
