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

use error::invalid_identifier_value;
use lazy_static::lazy_static;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types & Constants
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        self.0
    }
}

impl Identifier {
    pub fn new_sql_like(s: &str) -> Result<Self, error::Error> {
        lazy_static! {
            static ref SQL_IDENTIFIER: regex::Regex =
                regex::Regex::new(r"[\p{L}_][\p{L}\p{Nd}_]*").unwrap();
        }

        if is_valid_identifier_value(s, false, 128, &SQL_IDENTIFIER) {
            Ok(Self(s.to_string()))
        } else {
            Err(invalid_identifier_value(s.to_string()))
        }
    }

    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn is_valid_identifier_value(
    s: &str,
    allow_empty: bool,
    max_length: usize,
    regex: &regex::Regex,
) -> bool {
    s.is_empty() == allow_empty && s.len() < max_length && regex.is_match(s)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod ast;

pub mod data;

// pub mod eval;

pub mod error;

pub mod ops;

pub mod sort;

#[cfg(feature = "simple-data")]
pub mod simple;

//#[cfg(feature = "graphviz")]
pub mod graph;
