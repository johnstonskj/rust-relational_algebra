/*!
One-line description.

More detailed description, with

# Example

 */

use crate::data::{Relation, Tuple, Value};
use crate::simple::sort::SimpleSortRelation;
use crate::sort::SortRelation;
use std::{collections::HashSet, fmt::Display};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct SimpleRelation {
    schema: SimpleSortRelation,
    tuples: HashSet<SimpleTuple>,
}

#[derive(Debug)]
pub struct Tuples<'a> {
    iter: std::collections::hash_set::Iter<'a, SimpleTuple>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleTuple(Vec<Value>);

#[derive(Debug)]
pub struct Values<'a> {
    iter: std::slice::Iter<'a, Value>,
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

impl Display for SimpleRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{{...}}", self.schema().name())
    }
}

impl Relation for SimpleRelation {
    type Schema = SimpleSortRelation;
    type Item = SimpleTuple;

    fn schema(&self) -> &Self::Schema {
        &self.schema
    }

    fn tuples(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(Tuples {
            iter: self.tuples.iter(),
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for Tuples<'a> {
    type Item = &'a SimpleTuple;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SimpleTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.values()
                .map(Value::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Tuple for SimpleTuple {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn value(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }

    fn values(&self) -> Box<dyn Iterator<Item = &Value> + '_> {
        Box::new(Values {
            iter: self.0.iter(),
        })
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for Values<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
