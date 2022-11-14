/*!
This module provides a simplified model (AST) of the Relational Algebra with the traits
required to implement an evaluation environment.

The relational algebra is

## Definitions

* **Relational algebra**; the basic set of operations applicable within the relational model.
* **Relational algebra expression**; a sequence of relational algebra operations; such an expression can be considered a *query*.
* **Relation**; TDB
* **Attribute**; TBD
* **Attribute Schema**; TDB
* **Relation Schema**; TDB
* **Sort**; TBD
* **Tuple**; TDB
* **Domain**; TBD

## More Information

For more information on the Relational Algebra, see the following:

* [Relational Algebra](https://en.wikipedia.org/wiki/Relational_algebra) on Wikipedia.
* [Relational Algebra](http://infolab.stanford.edu/~ullman/fcdb/aut07/slides/ra.pdf) courseware from Stanford.
* [Relational Algebra and SQL](https://www.cs.cornell.edu/projects/btr/bioinformaticsschool/slides/gehrke.pdf) slides from Database Management Systems, Cornell.

Alternatively, try the tool [RelaX - relational algebra calculator](https://dbis-uibk.github.io/relax/landing), from the Institute of Computer Science at the University of Innsbruck.

# Package Organization

## Optional Features

* `graphviz` - include the ability to create a DOT graph from the AST.
* `simple_data` - include the ability to read CSV and JSON files as relation tuples.

 */

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use error::{invalid_name_value, Error};
use lazy_static::lazy_static;
use std::{fmt::Display, str::FromStr};

// ------------------------------------------------------------------------------------------------
// Public Types & Constants
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref SQL_IDENTIFIER: regex::Regex =
                regex::Regex::new(r"[\p{L}_][\p{L}\p{Nd}_]*").unwrap();
        }

        if !s.is_empty() && s.len() < 128 && SQL_IDENTIFIER.is_match(s) {
            Ok(Self(s.to_string()))
        } else {
            Err(invalid_name_value(s.to_string()))
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Name> for String {
    fn from(v: Name) -> Self {
        v.0
    }
}

impl Name {
    pub fn new_unchecked(s: &str) -> Self {
        assert!(!s.is_empty());
        Self(s.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod ast;

pub mod data;

pub mod error;

#[cfg(feature = "evaluation")]
pub mod eval;

pub mod sort;

#[cfg(feature = "simple_data")]
pub mod simple;

#[cfg(feature = "graphviz")]
pub mod graph;
