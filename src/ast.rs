/*!
Provides the in-memory model for the Relational Algebra.

# Expressions and Operators

A relational operation generally comprises an operator and one or two (infix)
operands. is one of:

| =Operator                | =Symbol | =Infix | =Arguments   |
| ------------------------ | ------- | ------ | ------------ |
| set union                | `∪`     | Yes    | No           |
| set intersection         | `∩`     | Yes    | No           |
| set difference           | `∖`     | Yes    | No           |
| set symmetric difference | `△`     | Yes    | No           |
| set cartesian product    | `×`     | Yes    | No           |
| Selection                | `σ`     | No     | Criteria     |
| Projection               | `π`     | No     | *Attributes* |
| Rename                   | `ρ`     | No     | Attributes   |
| Order                    | `τ`     | No     | Attributes   |
| Group                    | `γ`     | No     | Attributes   |
| natural join             | `⨝`     | Yes    | No           |
| theta join               | `⨝`     | Yes    | Criteria     |

A projection may also include constant values as well as attributes, and while
a projection with no attributes is valid it is represented in the AST as a
separate operator `Relation` with just the relation name.

*/

use crate::data::Value;
use crate::error::Error;
use crate::Name;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types & Constants
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Expression {
    name: Option<Name>,
    expr: RelationalOp,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionList(Vec<Expression>);

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum RelationalOp {
    Relation(Name),
    SetOperation(SetOperation),
    Selection(Selection),
    Projection(Projection),
    Rename(Rename),
    Order(Order),
    Group(Group),
    Join(Join),
}

// ------------------------------------------------------------------------------------------------

///
/// Denotes a set operation between two other relational operation.
///
#[derive(Clone, Debug, PartialEq)]
pub struct SetOperation {
    lhs: Box<RelationalOp>,
    op: SetOperator,
    rhs: Box<RelationalOp>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetOperator {
    /// Results in the union, `∪`, of two sets.
    Union,
    /// Results in the intersection, `∩`, of two sets.
    Intersection,
    /// Results in the difference, `∖`, of two sets.
    Difference,
    /// Results in the symmetric difference, `△`, of two sets.
    SymmetricDifference,
    /// Results in the cartesian product, `×`, of two sets.
    CartesianProduct,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Selection {
    criteria: Term,
    rhs: Box<RelationalOp>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Attribute {
    Index(usize),
    Name(Name),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Constant(Value),
    Exists(Attribute),
    Atom(Atom),
    Negate(Box<Term>),
    And(Box<Term>, Box<Term>),
    Or(Box<Term>, Box<Term>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Atom {
    lhs: Attribute,
    op: ComparisonOperator,
    rhs: ProjectedAttribute,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    StringMatch,
    StringNotMatch,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Projection {
    attributes: Vec<ProjectedAttribute>,
    rhs: Box<RelationalOp>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectedAttribute {
    Index(usize),
    Name(Name),
    Constant(Value),
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Rename {
    renames: HashMap<Attribute, Name>,
    rhs: Box<RelationalOp>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    attributes: Vec<Attribute>,
    rhs: Box<RelationalOp>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    attributes: Vec<Attribute>,
    rhs: Box<RelationalOp>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub enum Join {
    Natural(NaturalJoin),
    Theta(ThetaJoin),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NaturalJoin {
    lhs: Box<RelationalOp>,
    rhs: Box<RelationalOp>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThetaJoin {
    lhs: Box<RelationalOp>,
    criteria: Term,
    rhs: Box<RelationalOp>,
}

// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DisplayFormat {
    ToStringUnicode,
    ToStringAscii,
    Latex,
    Html,
}

pub trait Format {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn format_relational(top_level: &RelationalOp, fmt: DisplayFormat) -> String {
    match fmt {
        DisplayFormat::ToStringUnicode | DisplayFormat::ToStringAscii => {
            top_level.to_formatted_string(fmt)
        }
        DisplayFormat::Latex => format!("${{{}}}$", top_level.to_formatted_string(fmt)),
        DisplayFormat::Html => format!("<math>{}</math>", top_level.to_formatted_string(fmt)),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! display_from_format {
    ($for_type:ty) => {
        impl Display for $for_type {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}",
                    if f.alternate() {
                        self.to_formatted_string(DisplayFormat::ToStringAscii)
                    } else {
                        self.to_formatted_string(DisplayFormat::ToStringUnicode)
                    }
                )
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations

// ------------------------------------------------------------------------------------------------

impl Format for Expression {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        let expr = self.expr.to_formatted_string(fmt);
        if let Some(name) = self.name() {
            format!(
                "{} {} {}",
                name,
                match fmt {
                    DisplayFormat::ToStringUnicode => "\u{2254}",
                    DisplayFormat::ToStringAscii => ":=",
                    DisplayFormat::Latex => "\\coloneqq",
                    DisplayFormat::Html => "&#x2254;",
                },
                expr
            )
        } else {
            expr
        }
    }
}

display_from_format!(Expression);

impl<T> From<T> for Expression
where
    T: Into<RelationalOp>,
{
    fn from(v: T) -> Self {
        Self::new(v.into())
    }
}

impl Expression {
    pub fn new<S>(expr: S) -> Self
    where
        S: Into<RelationalOp>,
    {
        Self {
            name: None,
            expr: expr.into(),
        }
    }

    pub fn named<S>(name: Name, expr: S) -> Self
    where
        S: Into<RelationalOp>,
    {
        Self {
            name: Some(name),
            expr: expr.into(),
        }
    }

    pub fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }

    pub fn expression(&self) -> &RelationalOp {
        &self.expr
    }
}

impl Display for ExpressionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let separator_string = if f.alternate() { "; " } else { ";\n" };
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|e| format!("{}{}", e, separator_string))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl<T> From<T> for ExpressionList
where
    T: Into<Expression>,
{
    fn from(v: T) -> Self {
        Self::from(vec![v])
    }
}

impl<T> From<Vec<T>> for ExpressionList
where
    T: Into<Expression>,
{
    fn from(v: Vec<T>) -> Self {
        Self(v.into_iter().map(|e| e.into()).collect())
    }
}

impl AsRef<Vec<Expression>> for ExpressionList {
    fn as_ref(&self) -> &Vec<Expression> {
        &self.0
    }
}

// ------------------------------------------------------------------------------------------------

impl Format for RelationalOp {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        match self {
            Self::Relation(v) => v.to_formatted_string(fmt),
            Self::SetOperation(v) => v.to_formatted_string(fmt),
            Self::Selection(v) => v.to_formatted_string(fmt),
            Self::Projection(v) => v.to_formatted_string(fmt),
            Self::Rename(v) => v.to_formatted_string(fmt),
            Self::Order(v) => v.to_formatted_string(fmt),
            Self::Group(v) => v.to_formatted_string(fmt),
            Self::Join(v) => v.to_formatted_string(fmt),
        }
    }
}

display_from_format!(RelationalOp);

impl From<Name> for RelationalOp {
    fn from(v: Name) -> Self {
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

impl From<NaturalJoin> for RelationalOp {
    fn from(v: NaturalJoin) -> Self {
        Self::Join(v.into())
    }
}

impl From<ThetaJoin> for RelationalOp {
    fn from(v: ThetaJoin) -> Self {
        Self::Join(v.into())
    }
}

impl From<Rename> for RelationalOp {
    fn from(v: Rename) -> Self {
        Self::Rename(v)
    }
}

impl From<Order> for RelationalOp {
    fn from(v: Order) -> Self {
        Self::Order(v)
    }
}

impl From<Group> for RelationalOp {
    fn from(v: Group) -> Self {
        Self::Group(v)
    }
}

impl RelationalOp {
    pub fn relation(s: &str) -> Result<Self, Error> {
        Ok(Name::from_str(s)?.into())
    }

    pub fn relation_unchecked(s: &str) -> Self {
        Name::new_unchecked(s).into()
    }

    pub fn is_relation(&self) -> bool {
        matches!(self, Self::Relation(_))
    }

    pub fn as_relation(&self) -> Option<&Name> {
        match self {
            Self::Relation(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn union<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<Self>,
        S2: Into<Self>,
    {
        SetOperation::union(lhs, rhs).into()
    }

    pub fn intersect<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<Self>,
        S2: Into<Self>,
    {
        SetOperation::intersection(lhs, rhs).into()
    }

    pub fn difference<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<Self>,
        S2: Into<Self>,
    {
        SetOperation::difference(lhs, rhs).into()
    }

    pub fn cartesian_product<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<Self>,
        S2: Into<Self>,
    {
        SetOperation::cartesian_product(lhs, rhs).into()
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

    // --------------------------------------------------------------------------------------------

    pub fn select<T, S>(criteria: T, from: S) -> Self
    where
        T: Into<Term>,
        S: Into<Self>,
    {
        Self::Selection(Selection::new(criteria.into(), from.into()))
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

    // --------------------------------------------------------------------------------------------

    pub fn project<S>(attributes: Vec<ProjectedAttribute>, rhs: S) -> Self
    where
        S: Into<Self>,
    {
        Self::Projection(Projection::new(attributes, rhs.into()))
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

    // --------------------------------------------------------------------------------------------

    pub fn rename<S>(renames: HashMap<Attribute, Name>, rhs: S) -> Result<Self, Error>
    where
        S: Into<Self>,
    {
        Ok(Rename::new(renames, rhs)?.into())
    }

    pub fn rename_by_index<S>(renames: Vec<Name>, rhs: S) -> Result<Self, Error>
    where
        S: Into<Self>,
    {
        Ok(Rename::new_indexed(renames, rhs)?.into())
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

    // --------------------------------------------------------------------------------------------

    pub fn sort_by<S>(attributes: Vec<Attribute>, rhs: S) -> Self
    where
        S: Into<Self>,
    {
        Self::Order(Order::new(attributes, rhs.into()))
    }

    pub fn is_sort_by(&self) -> bool {
        matches!(self, Self::Order(_))
    }

    pub fn as_sort_by(&self) -> Option<&Order> {
        match self {
            Self::Order(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn group_by<S>(attributes: Vec<Attribute>, rhs: S) -> Self
    where
        S: Into<Self>,
    {
        Self::Group(Group::new(attributes, rhs.into()))
    }

    pub fn is_group_by(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn as_group_by(&self) -> Option<&Group> {
        match self {
            Self::Group(v) => Some(v),
            _ => None,
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn natural_join<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<Self>,
        S2: Into<Self>,
    {
        Join::Natural(NaturalJoin::new(lhs, rhs)).into()
    }

    pub fn theta_join<S1, T, S2>(lhs: S1, criteria: T, rhs: S2) -> Self
    where
        S1: Into<Self>,
        T: Into<Term>,
        S2: Into<Self>,
    {
        Join::Theta(ThetaJoin::new(lhs, criteria, rhs)).into()
    }

    pub fn is_join(&self) -> bool {
        matches!(self, Self::Join(_))
    }

    pub fn is_natural_join(&self) -> bool {
        matches!(self, Self::Join(Join::Natural(_)))
    }

    pub fn is_theta_join(&self) -> bool {
        matches!(self, Self::Join(Join::Theta(_)))
    }

    pub fn as_join(&self) -> Option<&Join> {
        match self {
            Self::Join(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Format for Name {
    fn to_formatted_string(&self, _: DisplayFormat) -> String {
        self.to_string()
    }
}

// ------------------------------------------------------------------------------------------------

impl Format for SetOperation {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        format!(
            "{} {} {}",
            to_term_string(&self.lhs, fmt),
            self.op,
            to_term_string(&self.rhs, fmt)
        )
    }
}

display_from_format!(SetOperation);

impl SetOperation {
    pub fn new<S1, S2>(lhs: S1, op: SetOperator, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        S2: Into<RelationalOp>,
    {
        Self {
            lhs: Box::new(lhs.into()),
            op,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn union<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        S2: Into<RelationalOp>,
    {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::Union,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn intersection<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        S2: Into<RelationalOp>,
    {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::Intersection,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn difference<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        S2: Into<RelationalOp>,
    {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::Difference,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn symmetric_difference<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        S2: Into<RelationalOp>,
    {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::SymmetricDifference,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn cartesian_product<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        S2: Into<RelationalOp>,
    {
        Self {
            lhs: Box::new(lhs.into()),
            op: SetOperator::CartesianProduct,
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn lhs(&self) -> &RelationalOp {
        &self.lhs
    }

    pub fn operator(&self) -> SetOperator {
        self.op
    }

    pub fn is_union(&self) -> bool {
        self.op == SetOperator::Union
    }

    pub fn is_intersection(&self) -> bool {
        self.op == SetOperator::Intersection
    }

    pub fn is_difference(&self) -> bool {
        self.op == SetOperator::Difference
    }

    pub fn is_symmetric_difference(&self) -> bool {
        self.op == SetOperator::SymmetricDifference
    }

    pub fn is_cartesian_product(&self) -> bool {
        self.op == SetOperator::CartesianProduct
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

impl Format for SetOperator {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        match (self, fmt) {
            (Self::Union, DisplayFormat::ToStringUnicode) => "∪",
            (Self::Union, DisplayFormat::ToStringAscii) => "union",
            (Self::Union, DisplayFormat::Latex) => "\\cup",
            (Self::Union, DisplayFormat::Html) => "&cup;",
            (Self::Intersection, DisplayFormat::ToStringUnicode) => "∩",
            (Self::Intersection, DisplayFormat::ToStringAscii) => "intersect",
            (Self::Intersection, DisplayFormat::Latex) => "\\cap",
            (Self::Intersection, DisplayFormat::Html) => "&cap;",
            (Self::Difference, DisplayFormat::ToStringUnicode) => "∖",
            (Self::Difference, DisplayFormat::ToStringAscii) => "difference",
            (Self::Difference, DisplayFormat::Latex) => "\\setminus",
            (Self::Difference, DisplayFormat::Html) => "&setminus;",
            (Self::SymmetricDifference, DisplayFormat::ToStringUnicode) => "△",
            (Self::SymmetricDifference, DisplayFormat::ToStringAscii) => "symdifference",
            (Self::SymmetricDifference, DisplayFormat::Latex) => "\\triangle",
            (Self::SymmetricDifference, DisplayFormat::Html) => "&xutri;",
            (Self::CartesianProduct, DisplayFormat::ToStringUnicode) => "",
            (Self::CartesianProduct, DisplayFormat::ToStringAscii) => "product",
            (Self::CartesianProduct, DisplayFormat::Latex) => "\\times",
            (Self::CartesianProduct, DisplayFormat::Html) => "&times;",
        }
        .to_string()
    }
}

display_from_format!(SetOperator);

// ------------------------------------------------------------------------------------------------

impl Format for Selection {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        let criteria = self.criteria.to_formatted_string(fmt);
        let rhs = to_term_string(&self.rhs, fmt);
        match fmt {
            DisplayFormat::ToStringUnicode => format!("σ[{}]{}", criteria, rhs),
            DisplayFormat::ToStringAscii => format!("select[{}]{}", criteria, rhs),
            DisplayFormat::Latex => format!("\\sigma_{{{}}}{}", criteria, rhs),
            DisplayFormat::Html => format!("&sigma;<sub>{}</sub>{}", criteria, rhs),
        }
    }
}

display_from_format!(Selection);

impl Selection {
    pub fn new<T, S>(criteria: T, rhs: S) -> Self
    where
        T: Into<Term>,
        S: Into<RelationalOp>,
    {
        Self {
            criteria: criteria.into(),
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn criteria(&self) -> &Term {
        &self.criteria
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

// ------------------------------------------------------------------------------------------------

impl Format for Attribute {
    fn to_formatted_string(&self, _fmt: DisplayFormat) -> String {
        match self {
            Self::Index(v) => v.to_string(),
            Self::Name(v) => v.to_string(),
        }
    }
}

display_from_format!(Attribute);

impl From<usize> for Attribute {
    fn from(v: usize) -> Self {
        Self::Index(v)
    }
}

impl From<Name> for Attribute {
    fn from(v: Name) -> Self {
        Self::Name(v)
    }
}

impl Attribute {
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

    pub fn as_name(&self) -> Option<&Name> {
        match self {
            Self::Name(v) => Some(v),
            _ => None,
        }
    }
}

impl Format for Term {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        match (self, fmt) {
            (Self::Constant(v), _) => v.to_string(),
            (Self::Exists(a), _) => format!("?{}", a.to_formatted_string(fmt)),
            (Self::Atom(a), _) => a.to_formatted_string(fmt),
            (Self::Negate(a), DisplayFormat::ToStringUnicode) => {
                format!("¬{}", a.to_formatted_string(fmt))
            }
            (Self::Negate(a), DisplayFormat::ToStringAscii) => {
                format!("not {}", a.to_formatted_string(fmt))
            }
            (Self::Negate(a), DisplayFormat::Latex) => {
                format!("\\neg{}", a.to_formatted_string(fmt))
            }
            (Self::Negate(a), DisplayFormat::Html) => {
                format!("&not;{}", a.to_formatted_string(fmt))
            }
            (Self::And(l, r), DisplayFormat::ToStringUnicode) => format!(
                "{} ∧ {}",
                l.to_formatted_string(fmt),
                r.to_formatted_string(fmt)
            ),
            (Self::And(l, r), DisplayFormat::ToStringAscii) => format!(
                "{} and {}",
                l.to_formatted_string(fmt),
                r.to_formatted_string(fmt)
            ),
            (Self::And(l, r), DisplayFormat::Latex) => format!(
                "{} \\land {}",
                l.to_formatted_string(fmt),
                r.to_formatted_string(fmt)
            ),
            (Self::And(l, r), DisplayFormat::Html) => format!(
                "{} &and; {}",
                l.to_formatted_string(fmt),
                r.to_formatted_string(fmt)
            ),
            (Self::Or(l, r), DisplayFormat::ToStringUnicode) => format!(
                "{} ∨ {}",
                l.to_formatted_string(fmt),
                r.to_formatted_string(fmt)
            ),
            (Self::Or(l, r), DisplayFormat::ToStringAscii) => format!(
                "{} or {}",
                l.to_formatted_string(fmt),
                r.to_formatted_string(fmt)
            ),
            (Self::Or(l, r), DisplayFormat::Latex) => format!(
                "{} \\lor {}",
                l.to_formatted_string(fmt),
                r.to_formatted_string(fmt)
            ),
            (Self::Or(l, r), DisplayFormat::Html) => format!(
                "{} &or; {}",
                l.to_formatted_string(fmt),
                r.to_formatted_string(fmt)
            ),
        }
    }
}

display_from_format!(Term);

impl From<Value> for Term {
    fn from(v: Value) -> Self {
        Self::Constant(v)
    }
}

impl From<Attribute> for Term {
    fn from(v: Attribute) -> Self {
        Self::Exists(v)
    }
}

impl From<usize> for Term {
    fn from(v: usize) -> Self {
        Self::Exists(v.into())
    }
}

impl From<Name> for Term {
    fn from(v: Name) -> Self {
        Self::Exists(v.into())
    }
}

impl From<Atom> for Term {
    fn from(v: Atom) -> Self {
        Self::Atom(v)
    }
}

impl Term {
    pub fn constant<V>(value: V) -> Self
    where
        V: Into<Value>,
    {
        Self::Constant(value.into())
    }

    pub fn exists<A>(attribute: A) -> Self
    where
        A: Into<Attribute>,
    {
        Self::Exists(attribute.into())
    }

    pub fn equals<A, P>(lhs: A, rhs: P) -> Self
    where
        A: Into<Attribute>,
        P: Into<ProjectedAttribute>,
    {
        Self::Atom(Atom::new(lhs.into(), ComparisonOperator::Equal, rhs.into()))
    }

    pub fn not_equals<A, P>(lhs: A, rhs: P) -> Self
    where
        A: Into<Attribute>,
        P: Into<ProjectedAttribute>,
    {
        Self::Atom(Atom::new(
            lhs.into(),
            ComparisonOperator::NotEqual,
            rhs.into(),
        ))
    }

    pub fn less_than<A, P>(lhs: A, rhs: P) -> Self
    where
        A: Into<Attribute>,
        P: Into<ProjectedAttribute>,
    {
        Self::Atom(Atom::new(
            lhs.into(),
            ComparisonOperator::LessThan,
            rhs.into(),
        ))
    }

    pub fn less_than_or_equal<A, P>(lhs: A, rhs: P) -> Self
    where
        A: Into<Attribute>,
        P: Into<ProjectedAttribute>,
    {
        Self::Atom(Atom::new(
            lhs.into(),
            ComparisonOperator::LessThanOrEqual,
            rhs.into(),
        ))
    }

    pub fn greater_than<A, P>(lhs: A, rhs: P) -> Self
    where
        A: Into<Attribute>,
        P: Into<ProjectedAttribute>,
    {
        Self::Atom(Atom::new(
            lhs.into(),
            ComparisonOperator::GreaterThan,
            rhs.into(),
        ))
    }

    pub fn greater_than_or_equal<A, P>(lhs: A, rhs: P) -> Self
    where
        A: Into<Attribute>,
        P: Into<ProjectedAttribute>,
    {
        Self::Atom(Atom::new(
            lhs.into(),
            ComparisonOperator::GreaterThanOrEqual,
            rhs.into(),
        ))
    }

    pub fn string_match<A, P>(lhs: A, rhs: P) -> Self
    where
        A: Into<Attribute>,
        P: Into<ProjectedAttribute>,
    {
        Self::Atom(Atom::new(
            lhs.into(),
            ComparisonOperator::StringMatch,
            rhs.into(),
        ))
    }

    pub fn string_not_match<A, P>(lhs: A, rhs: P) -> Self
    where
        A: Into<Attribute>,
        P: Into<ProjectedAttribute>,
    {
        Self::Atom(Atom::new(
            lhs.into(),
            ComparisonOperator::StringNotMatch,
            rhs.into(),
        ))
    }

    pub fn and<T1, T2>(lhs: T1, rhs: T2) -> Self
    where
        T1: Into<Term>,
        T2: Into<Term>,
    {
        Term::And(Box::new(lhs.into()), Box::new(rhs.into()))
    }

    pub fn or<T1, T2>(lhs: T1, rhs: T2) -> Self
    where
        T1: Into<Term>,
        T2: Into<Term>,
    {
        Term::Or(Box::new(lhs.into()), Box::new(rhs.into()))
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

    pub fn is_exists(&self) -> bool {
        matches!(self, Self::Exists(_))
    }

    pub fn as_exists(&self) -> Option<&Attribute> {
        match self {
            Self::Exists(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_atom(&self) -> bool {
        matches!(self, Self::Atom(_))
    }

    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Self::Atom(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_negated(&self) -> bool {
        matches!(self, Self::Negate(_))
    }

    pub fn as_negated(&self) -> Option<&Term> {
        match self {
            Self::Negate(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_conjunction(&self) -> bool {
        matches!(self, Self::And(_, _))
    }

    pub fn as_conjunction(&self) -> Option<(&Term, &Term)> {
        match self {
            Self::And(l, r) => Some((l, r)),
            _ => None,
        }
    }

    pub fn is_disjunction(&self) -> bool {
        matches!(self, Self::Or(_, _))
    }

    pub fn as_disjunction(&self) -> Option<(&Term, &Term)> {
        match self {
            Self::Or(l, r) => Some((l, r)),
            _ => None,
        }
    }

    pub fn negate(self) -> Self {
        Term::Negate(Box::new(self))
    }
}

impl Format for Atom {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        format!(
            "{}{}{}",
            self.lhs.to_formatted_string(fmt),
            self.op.to_formatted_string(fmt),
            self.rhs.to_formatted_string(fmt)
        )
    }
}

display_from_format!(Atom);

impl Atom {
    pub fn new(lhs: Attribute, op: ComparisonOperator, rhs: ProjectedAttribute) -> Self {
        Self { lhs, op, rhs }
    }

    pub fn equals(lhs: Attribute, rhs: ProjectedAttribute) -> Self {
        Self::new(lhs, ComparisonOperator::Equal, rhs)
    }

    pub fn not_equals(lhs: Attribute, rhs: ProjectedAttribute) -> Self {
        Self::new(lhs, ComparisonOperator::NotEqual, rhs)
    }

    pub fn less_than(lhs: Attribute, rhs: ProjectedAttribute) -> Self {
        Self::new(lhs, ComparisonOperator::LessThan, rhs)
    }

    pub fn less_than_or_equal(lhs: Attribute, rhs: ProjectedAttribute) -> Self {
        Self::new(lhs, ComparisonOperator::LessThanOrEqual, rhs)
    }

    pub fn greater_than(lhs: Attribute, rhs: ProjectedAttribute) -> Self {
        Self::new(lhs, ComparisonOperator::GreaterThan, rhs)
    }

    pub fn greater_than_or_equal(lhs: Attribute, rhs: ProjectedAttribute) -> Self {
        Self::new(lhs, ComparisonOperator::GreaterThanOrEqual, rhs)
    }

    pub fn string_match(lhs: Attribute, rhs: ProjectedAttribute) -> Self {
        Self::new(lhs, ComparisonOperator::StringMatch, rhs)
    }

    pub fn string_not_match(lhs: Attribute, rhs: ProjectedAttribute) -> Self {
        Self::new(lhs, ComparisonOperator::StringNotMatch, rhs)
    }

    pub fn lhs(&self) -> &Attribute {
        &self.lhs
    }

    pub fn operator(&self) -> ComparisonOperator {
        self.op
    }

    pub fn rhs(&self) -> &ProjectedAttribute {
        &self.rhs
    }
}

impl Format for ComparisonOperator {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        match (self, fmt) {
            (Self::Equal, DisplayFormat::ToStringUnicode) => "=",
            (Self::Equal, DisplayFormat::ToStringAscii) => "=",
            (Self::Equal, DisplayFormat::Latex) => "=",
            (Self::Equal, DisplayFormat::Html) => "&equals;",
            (Self::NotEqual, DisplayFormat::ToStringUnicode) => "≠",
            (Self::NotEqual, DisplayFormat::ToStringAscii) => "/=",
            (Self::NotEqual, DisplayFormat::Latex) => "\\neq",
            (Self::NotEqual, DisplayFormat::Html) => "&ne;",
            (Self::LessThan, DisplayFormat::ToStringUnicode) => "<",
            (Self::LessThan, DisplayFormat::ToStringAscii) => "<",
            (Self::LessThan, DisplayFormat::Latex) => "<",
            (Self::LessThan, DisplayFormat::Html) => "<",
            (Self::LessThanOrEqual, DisplayFormat::ToStringUnicode) => "≤",
            (Self::LessThanOrEqual, DisplayFormat::ToStringAscii) => "<=",
            (Self::LessThanOrEqual, DisplayFormat::Latex) => "\\leq",
            (Self::LessThanOrEqual, DisplayFormat::Html) => "&le;",
            (Self::GreaterThan, DisplayFormat::ToStringUnicode) => ">",
            (Self::GreaterThan, DisplayFormat::ToStringAscii) => ">",
            (Self::GreaterThan, DisplayFormat::Latex) => ">",
            (Self::GreaterThan, DisplayFormat::Html) => ">",
            (Self::GreaterThanOrEqual, DisplayFormat::ToStringUnicode) => "≥",
            (Self::GreaterThanOrEqual, DisplayFormat::ToStringAscii) => ">=",
            (Self::GreaterThanOrEqual, DisplayFormat::Latex) => "\\geq",
            (Self::GreaterThanOrEqual, DisplayFormat::Html) => "&ge;",
            (Self::StringMatch, DisplayFormat::ToStringUnicode) => "~",
            (Self::StringMatch, DisplayFormat::ToStringAscii) => "~",
            (Self::StringMatch, DisplayFormat::Latex) => r"\\sim",
            (Self::StringMatch, DisplayFormat::Html) => "&sim;",
            (Self::StringNotMatch, DisplayFormat::ToStringUnicode) => "≁",
            (Self::StringNotMatch, DisplayFormat::ToStringAscii) => "/~",
            (Self::StringNotMatch, DisplayFormat::Latex) => r"\\nsim",
            (Self::StringNotMatch, DisplayFormat::Html) => "&nsim;",
        }
        .to_string()
    }
}

display_from_format!(ComparisonOperator);

impl ComparisonOperator {
    pub fn negate(&self) -> Self {
        match self {
            Self::Equal => Self::NotEqual,
            Self::NotEqual => Self::Equal,
            Self::LessThan => Self::GreaterThanOrEqual,
            Self::LessThanOrEqual => Self::GreaterThan,
            Self::GreaterThan => Self::LessThanOrEqual,
            Self::GreaterThanOrEqual => Self::GreaterThanOrEqual,
            Self::StringMatch => Self::StringNotMatch,
            Self::StringNotMatch => Self::StringMatch,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Format for Projection {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        let attributes = self
            .attributes
            .iter()
            .map(|attribute| attribute.to_formatted_string(fmt))
            .collect::<Vec<String>>()
            .join(", ");
        let rhs = to_term_string(&self.rhs, fmt);
        match fmt {
            DisplayFormat::ToStringUnicode => format!("π[{}]{}", attributes, rhs),
            DisplayFormat::ToStringAscii => format!("project[{}]{}", attributes, rhs),
            DisplayFormat::Latex => format!("\\pi_{{{}}}{}", attributes, rhs),
            DisplayFormat::Html => format!("&pi;<sub>{}</sub>{}", attributes, rhs),
        }
    }
}

display_from_format!(Projection);

impl Projection {
    pub fn new<S>(attributes: Vec<ProjectedAttribute>, from: S) -> Self
    where
        S: Into<RelationalOp>,
    {
        assert!(!attributes.is_empty());

        Self {
            attributes,
            rhs: Box::new(from.into()),
        }
    }

    pub fn count(&self) -> usize {
        self.attributes.len()
    }

    pub fn attributes(&self) -> impl Iterator<Item = &ProjectedAttribute> {
        self.attributes.iter()
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

impl Format for ProjectedAttribute {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        match self {
            ProjectedAttribute::Index(v) => v.to_string(),
            ProjectedAttribute::Name(v) => v.to_formatted_string(fmt),
            ProjectedAttribute::Constant(v) => v.to_string(),
        }
    }
}

display_from_format!(ProjectedAttribute);

impl From<usize> for ProjectedAttribute {
    fn from(v: usize) -> Self {
        Self::Index(v)
    }
}

impl From<Name> for ProjectedAttribute {
    fn from(v: Name) -> Self {
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

    pub fn as_name(&self) -> Option<&Name> {
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

impl Format for Order {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        let attributes = self
            .attributes
            .iter()
            .map(|attribute| attribute.to_formatted_string(fmt))
            .collect::<Vec<String>>()
            .join(", ");
        let rhs = to_term_string(&self.rhs, fmt);
        match fmt {
            DisplayFormat::ToStringUnicode => format!("τ[{}]{}", attributes, rhs),
            DisplayFormat::ToStringAscii => format!("sort[{}]{}", attributes, rhs),
            DisplayFormat::Latex => format!("\\tau_{{{}}}{}", attributes, rhs),
            DisplayFormat::Html => format!("&tau;<sub>{}</sub>{}", attributes, rhs),
        }
    }
}

display_from_format!(Order);

impl Order {
    pub fn new<S>(attributes: Vec<Attribute>, from: S) -> Self
    where
        S: Into<RelationalOp>,
    {
        assert!(!attributes.is_empty());

        Self {
            attributes,
            rhs: Box::new(from.into()),
        }
    }

    pub fn count(&self) -> usize {
        self.attributes.len()
    }

    pub fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.iter()
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

// ------------------------------------------------------------------------------------------------

impl Format for Group {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        let attributes = self
            .attributes
            .iter()
            .map(|attribute| attribute.to_formatted_string(fmt))
            .collect::<Vec<String>>()
            .join(", ");
        let rhs = to_term_string(&self.rhs, fmt);
        match fmt {
            DisplayFormat::ToStringUnicode => format!("γ[{}]{}", attributes, rhs),
            DisplayFormat::ToStringAscii => format!("group[{}]{}", attributes, rhs),
            DisplayFormat::Latex => format!("\\gamma_{{{}}}{}", attributes, rhs),
            DisplayFormat::Html => format!("&gamma;<sub>{}</sub>{}", attributes, rhs),
        }
    }
}

display_from_format!(Group);

impl Group {
    pub fn new<S>(attributes: Vec<Attribute>, from: S) -> Self
    where
        S: Into<RelationalOp>,
    {
        assert!(!attributes.is_empty());

        Self {
            attributes,
            rhs: Box::new(from.into()),
        }
    }

    pub fn count(&self) -> usize {
        self.attributes.len()
    }

    pub fn attributes(&self) -> impl Iterator<Item = &Attribute> {
        self.attributes.iter()
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

// ------------------------------------------------------------------------------------------------

impl Format for Rename {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        let renames = if self.renames.keys().all(Attribute::is_index) {
            (0..self.renames.len())
                .map(|i| self.renames.get(&Attribute::Index(i)).unwrap().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            self.renames
                .iter()
                .map(|(left, right)| format!("{}/{}", left, right))
                .collect::<Vec<String>>()
                .join(", ")
        };
        let rhs = to_term_string(&self.rhs, fmt);
        match fmt {
            DisplayFormat::ToStringUnicode => format!("ρ[{}]{}", renames, rhs),
            DisplayFormat::ToStringAscii => format!("rename[{}]{}", renames, rhs),
            DisplayFormat::Latex => format!("\\rho_{{{}}}{}", renames, rhs),
            DisplayFormat::Html => format!("&rho;<sub>{}</sub>{}", renames, rhs),
        }
    }
}

display_from_format!(Rename);

impl Rename {
    pub fn new<S>(renames: HashMap<Attribute, Name>, rhs: S) -> Result<Self, Error>
    where
        S: Into<RelationalOp>,
    {
        assert!(!renames.is_empty());
        let initial_len = renames.len();
        let unique_names: HashSet<&Name> = renames.values().collect();
        if unique_names.len() == initial_len {
            Ok(Self {
                renames,
                rhs: Box::new(rhs.into()),
            })
        } else {
            unimplemented!()
        }
    }

    pub fn new_indexed<S>(renames: Vec<Name>, rhs: S) -> Result<Self, Error>
    where
        S: Into<RelationalOp>,
    {
        assert!(!renames.is_empty());
        let initial_len = renames.len();
        let unique_names: HashSet<&Name> = renames.iter().collect();
        if unique_names.len() == initial_len {
            let renames: HashMap<Attribute, Name> = renames
                .into_iter()
                .enumerate()
                .map(|(index, name)| (Attribute::Index(index), name))
                .collect();
            Ok(Self {
                renames,
                rhs: Box::new(rhs.into()),
            })
        } else {
            unimplemented!()
        }
    }

    pub fn count(&self) -> usize {
        self.renames.len()
    }

    pub fn renames(&self) -> impl Iterator<Item = (&Attribute, &Name)> {
        self.renames.iter()
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

// ------------------------------------------------------------------------------------------------

impl Format for Join {
    fn to_formatted_string(&self, _fmt: DisplayFormat) -> String {
        match self {
            Self::Natural(v) => v.to_string(),
            Self::Theta(v) => v.to_string(),
        }
    }
}

display_from_format!(Join);

impl From<NaturalJoin> for Join {
    fn from(v: NaturalJoin) -> Self {
        Self::Natural(v)
    }
}

impl From<ThetaJoin> for Join {
    fn from(v: ThetaJoin) -> Self {
        Self::Theta(v)
    }
}

impl Join {
    pub fn natural<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        S2: Into<RelationalOp>,
    {
        Self::Natural(NaturalJoin::new(lhs.into(), rhs.into()))
    }

    pub fn is_natural(&self) -> bool {
        matches!(self, Self::Natural(_))
    }

    pub fn as_natural(&self) -> Option<&NaturalJoin> {
        match self {
            Self::Natural(v) => Some(v),
            _ => None,
        }
    }

    pub fn theta<S1, T, S2>(lhs: S1, criteria: T, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        T: Into<Term>,
        S2: Into<RelationalOp>,
    {
        Self::Theta(ThetaJoin::new(lhs.into(), criteria.into(), rhs.into()))
    }

    pub fn is_theta(&self) -> bool {
        matches!(self, Self::Theta(_))
    }

    pub fn as_theta(&self) -> Option<&ThetaJoin> {
        match self {
            Self::Theta(v) => Some(v),
            _ => None,
        }
    }
}

impl Format for NaturalJoin {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        format!(
            "{} {} {}",
            to_term_string(&self.lhs, fmt),
            match fmt {
                DisplayFormat::ToStringUnicode => "⨝",
                DisplayFormat::ToStringAscii => "join",
                DisplayFormat::Latex => "\\Join",
                DisplayFormat::Html => "⨝",
            },
            to_term_string(&self.rhs, fmt)
        )
    }
}

display_from_format!(NaturalJoin);

impl NaturalJoin {
    pub fn new<S1, S2>(lhs: S1, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        S2: Into<RelationalOp>,
    {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn lhs(&self) -> &RelationalOp {
        &self.lhs
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

impl Format for ThetaJoin {
    fn to_formatted_string(&self, fmt: DisplayFormat) -> String {
        format!(
            "{} {}{} {}",
            to_term_string(&self.lhs, fmt),
            match fmt {
                DisplayFormat::ToStringUnicode => "⨝",
                DisplayFormat::ToStringAscii => "theta",
                DisplayFormat::Latex => "\\Join",
                DisplayFormat::Html => "⨝",
            },
            match fmt {
                DisplayFormat::ToStringUnicode | DisplayFormat::ToStringAscii =>
                    format!("[{}]", self.criteria.to_formatted_string(fmt)),
                DisplayFormat::Latex => format!("{{{}}}", self.criteria.to_formatted_string(fmt)),
                DisplayFormat::Html =>
                    format!("<sub>{}</sub>", self.criteria.to_formatted_string(fmt)),
            },
            to_term_string(&self.rhs, fmt)
        )
    }
}

display_from_format!(ThetaJoin);

impl ThetaJoin {
    pub fn new<S1, T, S2>(lhs: S1, criteria: T, rhs: S2) -> Self
    where
        S1: Into<RelationalOp>,
        T: Into<Term>,
        S2: Into<RelationalOp>,
    {
        Self {
            lhs: Box::new(lhs.into()),
            criteria: criteria.into(),
            rhs: Box::new(rhs.into()),
        }
    }

    pub fn is_equi_join(&self) -> bool {
        unimplemented!()
    }

    pub fn lhs(&self) -> &RelationalOp {
        &self.lhs
    }

    pub fn criteria(&self) -> &Term {
        &self.criteria
    }

    pub fn rhs(&self) -> &RelationalOp {
        &self.rhs
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn to_term_string(r: &RelationalOp, fmt: DisplayFormat) -> String {
    if r.is_relation() {
        r.to_string()
    } else if fmt == DisplayFormat::Latex {
        format!("\\({}\\)", r)
    } else {
        format!("({})", r)
    }
}
