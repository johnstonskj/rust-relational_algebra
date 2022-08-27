/*!
This module provides a simplified model of the Relational Algebra and includes the capability to
compile rules and atoms into [relational operations](RelationalOp).

![module UML](https://raw.githubusercontent.com/johnstonskj/rust-asdi/main/book/src/model/idb_query_relational.svg)

# Example

# Features

* `graphviz`
* `json`

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

impl Into<String> for Name {
    fn into(self) -> String {
        self.0
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

pub mod ops;

pub mod sort;

#[cfg(feature = "simple-data")]
pub mod simple;

#[cfg(feature = "graphviz")]
pub mod graph;
