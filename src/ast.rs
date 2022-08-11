/*!


*/

use crate::data::Value;
use crate::Identifier;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

// ------------------------------------------------------------------------------------------------
// Public Types & Constants
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum RelationalOp {
    Relation(Identifier),
    SetOperation(SetOperation),
    Selection(Selection),
    Projection(Projection),
    Rename(Rename),
    Join(Join),
    Assignment(Assignment),
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct SetOperation {
    lhs: Box<RelationalOp>,
    op: SetOperator,
    rhs: Box<RelationalOp>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SetOperator {
    Union,
    Intersection,
    Difference,
    CartesianProduct,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Selection {
    source: Box<RelationalOp>,
    criteria: Vec<Criteria>,
    negated: bool,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    source: Box<RelationalOp>,
    attributes: Vec<ProjectedAttribute>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectedAttribute {
    Index(usize),
    Name(Identifier),
    Constant(Value),
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Rename(HashMap<usize, Identifier>);

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Join {
    lhs: Box<RelationalOp>,
    criteria: Vec<Criteria>,
    rhs: Box<RelationalOp>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Criteria {
    index: usize,
    op: ComparisonOperator,
    value: ProjectedAttribute,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    StringMatch,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    name: Identifier,
    rhs: Box<RelationalOp>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for RelationalOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Relation(v) => v.to_string(),
                Self::SetOperation(v) => v.to_string(),
                Self::Selection(v) => v.to_string(),
                Self::Projection(v) => v.to_string(),
                Self::Rename(v) => v.to_string(),
                Self::Join(v) => v.to_string(),
                Self::Assignment(v) => v.to_string(),
            }
        )
    }
}

impl From<Identifier> for RelationalOp {
    fn from(v: Identifier) -> Self {
        Self::Relation(v)
    }
}

impl From<SetOperation> for RelationalOp {
    fn from(v: SetOperation) -> Self {
        Self::SetOperation(v)
    }
}

impl From<Selection> for RelationalOp {
    fn from(v: Selection) -> Self {
        Self::Selection(v)
    }
}

impl From<Projection> for RelationalOp {
    fn from(v: Projection) -> Self {
        Self::Projection(v)
    }
}

impl From<Join> for RelationalOp {
    fn from(v: Join) -> Self {
        Self::Join(v)
    }
}

impl From<Assignment> for RelationalOp {
    fn from(v: Assignment) -> Self {
        Self::Assignment(v)
    }
}

impl RelationalOp {
    pub fn is_relation(&self) -> bool {
        matches!(self, Self::Relation(_))
    }

    pub fn as_relation(&self) -> Option<&Identifier> {
        match self {
            Self::Relation(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_set_operation(&self) -> bool {
        matches!(self, Self::SetOperation(_))
    }

    pub fn as_set_operation(&self) -> Option<&SetOperation> {
        match self {
            Self::SetOperation(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_selection(&self) -> bool {
        matches!(self, Self::Selection(_))
    }

    pub fn as_selection(&self) -> Option<&Selection> {
        match self {
            Self::Selection(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_projection(&self) -> bool {
        matches!(self, Self::Projection(_))
    }

    pub fn as_projection(&self) -> Option<&Projection> {
        match self {
            Self::Projection(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_rename(&self) -> bool {
        matches!(self, Self::Rename(_))
    }

    pub fn as_rename(&self) -> Option<&Rename> {
        match self {
            Self::Rename(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_join(&self) -> bool {
        matches!(self, Self::Join(_))
    }

    pub fn as_join(&self) -> Option<&Join> {
        match self {
            Self::Join(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_assignment(&self) -> bool {
        matches!(self, Self::Assignment(_))
    }

    pub fn as_assignment(&self) -> Option<&Assignment> {
        match self {
            Self::Assignment(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for SetOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {} ({})", self.lhs, self.op, self.rhs)
    }
}

impl SetOperation {
    pub fn new<S: Into<RelationalOp>>(lhs: S, op: SetOperator, rhs: S) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            op,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn new_union<S: Into<RelationalOp>>(lhs: S, rhs: S) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::Union,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn new_intersection<S: Into<RelationalOp>>(lhs: S, rhs: S) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::Intersection,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn new_difference<S: Into<RelationalOp>>(lhs: S, rhs: S) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::Difference,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn new_cartesian_product<S: Into<RelationalOp>>(lhs: S, rhs: S) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::CartesianProduct,
            rhs: Box::new(rhs.into()),
        }
    }
}

const UNION_OPERATOR: &str = "∪";
const UNION_OPERATOR_ALT: &str = "union";
const INTERSECT_OPERATOR: &str = "∩";
const INTERSECT_OPERATOR_ALT: &str = "intersect";
const DIFFERENCE_OPERATOR: &str = "∖";
const DIFFERENCE_OPERATOR_ALT: &str = "difference";
const PRODUCT_OPERATOR: &str = "⨯";
const PRODUCT_OPERATOR_ALT: &str = "product";

impl Display for SetOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{}",
                match self {
                    Self::Union => "union",
                    Self::Intersection => "intersect",
                    Self::Difference => "difference",
                    Self::CartesianProduct => "product",
                }
            )
        } else {
            write!(
                f,
                "{}",
                match self {
                    Self::Union => "∪",
                    Self::Intersection => "∩",
                    Self::Difference => "∖",
                    Self::CartesianProduct => "⨯",
                }
            )
        }
    }
}

// ------------------------------------------------------------------------------------------------

const SELECT_OPERATOR: &str = "σ";
const SELECT_OPERATOR_ALT: &str = "select";

impl Display for Selection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_all() {
            write!(f, "{}", self.source)
        } else {
            write!(
                f,
                "σ[{}]({})",
                self.criteria
                    .iter()
                    .map(Criteria::to_string)
                    .collect::<Vec<String>>()
                    .join(", "),
                self.source
            )
        }
    }
}

impl Selection {
    pub fn new<V: Into<Vec<Criteria>>, S: Into<RelationalOp>>(
        criteria: V,
        from: S,
        negated: bool,
    ) -> Self {
        Self {
            source: Box::new(from.into()),
            criteria: criteria.into(),
            negated,
        }
    }

    pub fn all<S: Into<RelationalOp>>(from: S) -> Self {
        Self {
            source: Box::new(from.into()),
            criteria: Default::default(),
            negated: false,
        }
    }

    pub fn is_all(&self) -> bool {
        self.criteria.is_empty()
    }

    pub fn is_negated(&self) -> bool {
        self.negated
    }

    pub fn criteria_count(&self) -> usize {
        self.criteria.len()
    }

    pub fn criteria(&self) -> impl Iterator<Item = &'_ Criteria> {
        self.criteria.iter()
    }

    pub fn contains(&self, value: &Criteria) -> bool {
        self.criteria.contains(value)
    }

    pub fn add(&mut self, criteria: Criteria) {
        self.criteria.push(criteria);
    }
}

// ------------------------------------------------------------------------------------------------

const PROJECT_OPERATOR: &str = "Π";
const PROJECT_OPERATOR_ALT: &str = "project";

impl Display for Projection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_all() {
            write!(f, "{}", self.source)
        } else {
            write!(
                f,
                "Π[{}]({})",
                self.attributes
                    .iter()
                    .map(|attribute| attribute.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                self.source
            )
        }
    }
}

impl Projection {
    pub fn new<V: Into<Vec<ProjectedAttribute>>, S: Into<RelationalOp>>(
        attributes: V,
        from: S,
    ) -> Self {
        Self {
            source: Box::new(from.into()),
            attributes: attributes.into(),
        }
    }

    pub fn all<S: Into<RelationalOp>>(from: S) -> Self {
        Self {
            source: Box::new(from.into()),
            attributes: Default::default(),
        }
    }

    pub fn is_all(&self) -> bool {
        self.attributes.is_empty()
    }

    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    pub fn attributes(&self) -> impl Iterator<Item = &ProjectedAttribute> {
        self.attributes.iter()
    }
}

impl Display for ProjectedAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProjectedAttribute::Index(v) => v.to_string(),
                ProjectedAttribute::Name(v) => v.to_string(),
                ProjectedAttribute::Constant(v) => v.to_string(),
            }
        )
    }
}

impl From<usize> for ProjectedAttribute {
    fn from(v: usize) -> Self {
        Self::Index(v)
    }
}

impl From<Identifier> for ProjectedAttribute {
    fn from(v: Identifier) -> Self {
        Self::Name(v)
    }
}

impl From<Value> for ProjectedAttribute {
    fn from(v: Value) -> Self {
        Self::Constant(v)
    }
}

impl ProjectedAttribute {
    pub fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }

    pub fn as_index(&self) -> Option<usize> {
        match self {
            Self::Index(v) => Some(*v),
            _ => None,
        }
    }

    pub fn is_name(&self) -> bool {
        matches!(self, Self::Name(_))
    }

    pub fn as_name(&self) -> Option<&Identifier> {
        match self {
            Self::Name(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    pub fn as_constant(&self) -> Option<&Value> {
        match self {
            Self::Constant(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

const RENAME_OPERATOR: &str = "ρ";
const RENAME_OPERATOR_ALT: &str = "rename";

impl Display for Rename {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TryFrom<HashMap<usize, Identifier>> for Rename {
    type Error = crate::error::Error;

    fn try_from(value: HashMap<usize, Identifier>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<Vec<Identifier>> for Rename {
    type Error = crate::error::Error;

    fn try_from(value: Vec<Identifier>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Rename {
    pub fn names(&self) -> impl Iterator<Item = (usize, &Identifier)> {
        self.0.iter().map(|(i, v)| (*i, v))
    }
}

// ------------------------------------------------------------------------------------------------

const NATURAL_JOIN_OPERATOR: &str = "⨝";
const NATURAL_JOIN_OPERATOR_ALT: &str = "join";
const THETA_JOIN_OPERATOR: &str = "⨝𝞱";
const THETA_JOIN_OPERATOR_ALT: &str = "theta";

impl Display for Join {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_natural() {
            write!(f, "({}) ⨝  ({})", self.lhs, self.rhs)
        } else {
            write!(
                f,
                "({}) ⨝𝞱[{}] ({})",
                self.lhs,
                self.criteria
                    .iter()
                    .map(Criteria::to_string)
                    .collect::<Vec<String>>()
                    .join(", "),
                self.rhs
            )
        }
    }
}

impl Join {
    pub fn natural<S: Into<RelationalOp>>(lhs: S, rhs: S) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            criteria: Default::default(),
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn theta<S: Into<RelationalOp>, V: Into<Vec<Criteria>>>(
        lhs: S,
        criteria: V,
        rhs: S,
    ) -> Self {
        let criteria = criteria.into();
        assert!(!criteria.is_empty());
        Self {
            lhs: Box::new(lhs.into()),
            criteria,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn is_natural(&self) -> bool {
        self.criteria.is_empty()
    }

    pub fn is_theta(&self) -> bool {
        !self.criteria.is_empty()
    }

    pub fn lhs(&self) -> &RelationalOp {
        &self.lhs
    }

    pub fn critera(&self) -> impl Iterator<Item = &Criteria> {
        self.criteria.iter()
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

impl Display for Criteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.index, self.op, self.value)
    }
}

impl Criteria {
    pub fn new(index: usize, op: ComparisonOperator, value: ProjectedAttribute) -> Self {
        Self { index, op, value }
    }

    pub fn attribute_index(&self) -> usize {
        self.index
    }

    pub fn operator(&self) -> ComparisonOperator {
        self.op
    }

    pub fn compare_to(&self) -> &ProjectedAttribute {
        &self.value
    }
}

const EQUALITY_OPERATOR: &str = "=";
const EQUALITY_OPERATOR_ALT: &str = "=";
const INEQUALITY_OPERATOR: &str = "≠";
const INEQUALITY_OPERATOR_ALT: &str = "/=";
const LESS_THAN_OPERATOR: &str = "<";
const LESS_THAN_OPERATOR_ALT: &str = "<";
const LESS_THAN_OR_EQUAL_OPERATOR: &str = "≤";
const LESS_THAN_OR_EQUAL_OPERATOR_ALT: &str = "<=";
const GREATER_THAN_OPERATOR: &str = ">";
const GREATER_THAN_OPERATOR_ALT: &str = ">";
const GREATER_THAN_OR_EQUAL_OPERATOR: &str = "≥";
const GREATER_THAN_OR_EQUAL_OPERATOR_ALT: &str = ">=";
const STRING_MATCH_OPERATOR: &str = "∼";
const STRING_MATCH_OPERATOR_ALT: &str = "~=";

impl Display for ComparisonOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equal => EQUALITY_OPERATOR,
                Self::NotEqual => INEQUALITY_OPERATOR,
                Self::LessThan => LESS_THAN_OPERATOR,
                Self::LessThanOrEqual => LESS_THAN_OR_EQUAL_OPERATOR,
                Self::GreaterThan => GREATER_THAN_OPERATOR,
                Self::GreaterThanOrEqual => GREATER_THAN_OR_EQUAL_OPERATOR,
                Self::StringMatch => STRING_MATCH_OPERATOR,
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------

const ASSIGNMENT_OPERATOR: &str = "≔";
const ASSIGNMENT_OPERATOR_ALT: &str = ":=";

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.name, ASSIGNMENT_OPERATOR, self.rhs)
    }
}

impl Assignment {
    pub fn new(name: Identifier, rhs: RelationalOp) -> Self {
        Self {
            name,
            rhs: Box::new(rhs),
        }
    }
}
