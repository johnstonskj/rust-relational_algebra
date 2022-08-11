/*!
TBD
 */

use std::collections::HashMap;

use crate::ast::Criteria;
use crate::Identifier;

// ------------------------------------------------------------------------------------------------
// Public Types & Constants
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Index(usize),
    Name(Box<Identifier>),
}

#[doc(alias = "∩")]
pub trait Intersect<Rhs = Self> {
    type Output;

    fn intersect(self, rhs: Rhs) -> Self::Output;
}

#[doc(alias = "∪")]
pub trait Union<Rhs = Self> {
    type Output;

    fn union(self, rhs: Rhs) -> Self::Output;
}

#[doc(alias = "∖")]
pub trait Difference<Rhs = Self> {
    type Output;

    fn difference(self, rhs: Rhs) -> Self::Output;
}

#[doc(alias = "×")]
pub trait CartesianProduct<Rhs = Self> {
    type Output;

    fn cartesian_product(self, rhs: Rhs) -> Self::Output;
}

#[doc(alias = "σ")]
pub trait Select {
    type Output;

    fn select(self, criteria: &[Criteria]) -> Self::Output;
}

#[doc(alias = "Π")]
pub trait Project {
    type Output;

    fn project(self, indices: &[Attribute]) -> Self::Output;
}

#[doc(alias = "⨝")]
pub trait NaturalJoin<Rhs = Self> {
    type Output;

    fn natural_join(self, rhs: Rhs) -> Self::Output;
}

#[doc(alias = "θ")]
pub trait ThetaJoin<Rhs = Self> {
    type Output;

    fn theta_join(self, criteria: &[Criteria], rhs: Rhs) -> Self::Output;
}

#[doc(alias = "ρ")]
pub trait Rename<Rhs = Self> {
    type Output;

    fn rename(self, source: Attribute, target: Box<Identifier>) -> Self::Output;

    fn rename_all(self, names: HashMap<Attribute, Box<Identifier>>) -> Self::Output;
}
