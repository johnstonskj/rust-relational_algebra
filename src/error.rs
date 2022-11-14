/*!
Provides the [`Error`] and [`Result`] types and error constructor functions.
 */

use std::fmt::Display;

use crate::data::Value;
use crate::sort::Domain;
use crate::Name;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The type for all errors returned from this library.
#[derive(Debug)]
pub enum Error {
    /// The named relation does not exist in the selected database
    InvalidName { value: String },

    /// The named relation does not exist in the selected database
    RelationDoesNotExist { name: Name },

    /// The named attribute was not a member of the relation or view schema.
    AttributeDoesNotExist { name: Name },

    /// The attribute index is not valid for the relation or view schema.
    AttributeIndexInvalid { index: usize },

    /// A requested operation cannot be performed as the values have incompatible types.
    IncompatibleTypes {
        lhs_domain: Domain,
        rhs_domain: Domain,
    },

    /// The value is not a valid representation for the expected type.
    InvalidValue {
        expecting_domain: Domain,
        given_value: Value,
    },

    /// The arity of facts must be greater than, or equal to, 1.
    NullaryFactsNotAllowed,
}

///
/// The result of operations where the error returned is `crate::error::Error`.
///
pub type Result<T> = std::result::Result<T, Error>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn invalid_name_value<S>(value: S) -> Error
where
    S: Into<String>,
{
    Error::InvalidName {
        value: value.into(),
    }
}

#[inline]
pub fn relation_does_not_exist(name: Name) -> Error {
    Error::RelationDoesNotExist { name }
}

/// The attribute does not exist in the selected relation or view schema
#[inline]
pub fn attribute_does_not_exist(name: Name) -> Error {
    Error::AttributeDoesNotExist { name }
}

/// The attribute index is not valid for the relation or view schema.
#[inline]
pub fn attribute_index_invalid(index: usize) -> Error {
    Error::AttributeIndexInvalid { index }
}

/// The arity of facts must be greater than, or equal to, 1.
#[inline]
pub fn nullary_facts_not_allowed() -> Error {
    Error::NullaryFactsNotAllowed
}

/// A requested operation cannot be performed as the values have incompatible types.
#[inline]
pub fn incompatible_types(lhs_domain: Domain, rhs_domain: Domain) -> Error {
    Error::IncompatibleTypes {
        lhs_domain,
        rhs_domain,
    }
}

/// A value provided is not valid for the expected type.
#[inline]
pub fn invalid_value<V>(expecting_domain: Domain, given_value: V) -> Error
where
    V: Into<Value>,
{
    Error::InvalidValue {
        expecting_domain,
        given_value: given_value.into(),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidName { value } => {
                    format!("The value `{}` provided is not a legal name.", value)
                }
                Error::RelationDoesNotExist { name } => {
                    format!("The relation named `{}` does not exist.", name)
                }
                Error::AttributeDoesNotExist { name } => {
                    format!("The attribute named `{}` does not exist.", name)
                }
                Error::AttributeIndexInvalid { index } => {
                    format!("The attribute index `{}` does not exist.", index)
                }
                Error::IncompatibleTypes { lhs_domain, rhs_domain } => format!(
                    "The attempted operation cannot be performed as the values have incompatible types (`{}`, `{}`).",
                    lhs_domain, rhs_domain
                ),
                Error::InvalidValue {
                    expecting_domain,
                    given_value,
                } => format!(
                    "The value `{}` is not a valid `{}`.",
                    given_value, expecting_domain
                ),
                Error::NullaryFactsNotAllowed => "Nullary facts are not allowed".to_string(),
            }
        )
    }
}

impl std::error::Error for Error {}

impl<T> From<Error> for Result<T> {
    fn from(e: Error) -> Self {
        Err(e)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
