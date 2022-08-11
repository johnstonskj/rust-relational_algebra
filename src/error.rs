/*!
One-line description.

More detailed description, with

# Example

 */

use crate::data::{DataType, Value};
use crate::Identifier;

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
    InvalidIdentifier { value: String },

    /// The named relation does not exist in the selected database
    RelationDoesNotExist { name: Identifier },

    /// The named attribute was not a member of the relation or view schema.
    AttributeDoesNotExist { name: Identifier },

    /// The attribute index is not valid for the relation or view schema.
    AttributeIndexInvalid { index: usize },

    /// A requested operation cannot be performed as the values have incompatible types.
    IncompatibleTypes {
        lhs_type: DataType,
        rhs_type: DataType,
    },

    /// The value is not a valid representation for the expected type.
    InvalidValue {
        expecting_type: DataType,
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
pub fn invalid_identifier_value<S>(value: S) -> Error
where
    S: Into<String>,
{
    Error::InvalidIdentifier {
        value: value.into(),
    }
}

#[inline]
pub fn relation_does_not_exist(name: Identifier) -> Error {
    Error::RelationDoesNotExist { name }
}

/// The attribute does not exist in the selected relation or view schema
#[inline]
pub fn attribute_does_not_exist(name: Identifier) -> Error {
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
pub fn incompatible_types(lhs_type: DataType, rhs_type: DataType) -> Error {
    Error::IncompatibleTypes { lhs_type, rhs_type }
}

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
