/*!
Provides an implementation of a query analyzer and execution model for Expressions.

# Example

 */

use crate::{
    ast::{ComparisonOperator, Criteria, Group, Order, ProjectedAttribute, Selection},
    data::Value,
};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn analyze_expression() -> Result {}

pub fn evaluate_expression() -> Result<Box<dyn Relation>> {
    todo!()
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

//
// impl RelationalOp {
//     pub fn compile_atom(atom: &Atom, project_constants: bool) -> Result<Self> {
//         Self::compile_atom_with(atom, project_constants, Default::default())
//     }
//
//     pub fn compile_atom_with(
//         atom: &Atom,
//         project_constants: bool,
//         criteria: Vec<Criteria>,
//     ) -> Result<Self> {
//         println!(
//             "compile_atom_with > {} ({}) {:?}",
//             atom,
//             if project_constants {
//                 "project constants"
//             } else {
//                 "drop constants"
//             },
//             criteria
//         );
//         let projections: Vec<Attribute<Variable>> = atom
//             .iter()
//             .enumerate()
//             .filter(|(_, term)| {
//                 if project_constants {
//                     !term.is_anonymous()
//                 } else {
//                     term.is_variable()
//                 }
//             })
//             .map(|(i, term)| {
//                 let mut attribute = match term {
//                     Term::Variable(v) => Attribute::from(v.clone()),
//                     Term::Constant(v) => Attribute::typed(v.kind()),
//                     Term::Anonymous => unreachable!(),
//                 };
//                 attribute.set_index(i);
//                 attribute
//             })
//             .collect();
//         println!("compile_atom_with > project {:?}", projections);
//
//         if projections.is_empty() {
//             Err(nullary_facts_not_allowed())
//         } else {
//             let mut static_criteria: Vec<Criteria> = atom
//                 .iter()
//                 .enumerate()
//                 .filter_map(|(i, term)| term.as_constant().map(|c| (i, c)))
//                 .map(|(i, constant)| Criteria {
//                     index: i,
//                     op: ComparisonOperator::Equal,
//                     value: CriteriaValue::Value(constant.clone()),
//                 })
//                 .collect();
//             println!("compile_atom_with > static_criteria {:?}", static_criteria);
//             static_criteria.extend(criteria.into_iter());
//             Ok(
//                 match (
//                     project_constants,
//                     projections.len() == atom.len(), // true if we projection is complete
//                     static_criteria.is_empty(),
//                 ) {
//                     (_, true, true) => RelationalOp::Relation(atom.label_ref().into()),
//                     (_, true, false) => RelationalOp::Selection(Selection::new(
//                         static_criteria,
//                         RelationalOp::Relation(atom.label_ref().into()),
//                         false,
//                     )),
//                     (true, false, false) => RelationalOp::Selection(Selection::new(
//                         static_criteria,
//                         RelationalOp::Projection(Projection::new(
//                             projections,
//                             RelationalOp::Relation(atom.label_ref().into()),
//                         )),
//                         false,
//                     )),
//                     (false, false, false) => RelationalOp::Projection(Projection::new(
//                         projections,
//                         RelationalOp::Selection(Selection::new(
//                             static_criteria,
//                             RelationalOp::Relation(atom.label_ref().into()),
//                             false,
//                         )),
//                     )),
//                     (false, false, true) => RelationalOp::Projection(Projection::new(
//                         projections,
//                         RelationalOp::Relation(atom.label_ref().into()),
//                     )),
//                     state => {
//                         eprintln!("Unexpected state: {:?}", state);
//                         unreachable!()
//                     }
//                 },
//             )
//         }
//     }
//
//     pub fn compile_rule(rule: &Rule) -> Result<Self> {
//         println!("----------------------------------------------------------------------");
//         let arithmetic: Vec<(&Comparison, bool)> = rule
//             .literals()
//             .filter_map(|lit| lit.as_arithmetic().map(|atom| (atom, lit.is_negative())))
//             .collect();
//         let relational: Vec<(&Atom, bool)> = rule
//             .literals()
//             .filter_map(|lit| lit.as_relational().map(|comp| (comp, lit.is_negative())))
//             .collect();
//
//         // TODO: (ISSUE/rust-asdi/3) negation
//
//         let mut ops: Vec<RelationalOp> = Default::default();
//         let mut theta: Vec<&Comparison> = Default::default();
//         for (atom, atom_negated) in relational {
//             println!("compile_rule > atom {} (negated {})", atom, atom_negated);
//             let mut criteria: Vec<Criteria> = Default::default();
//             for (comparison, comparison_negated) in &arithmetic {
//                 println!(
//                     "compile_rule > atom > comparison {:?} (negated {})",
//                     comparison, comparison_negated
//                 );
//                 if let Err(e) = comparison.sanity_check() {
//                     warn!(
//                         "Ignoring arithmetic literal '{:?}', sanity check failed: {}",
//                         comparison, e
//                     );
//                 } else {
//                     match (comparison.lhs(), comparison.operator(), comparison.rhs()) {
//                         (Term::Variable(lhs), op, Term::Constant(rhs)) => {
//                             if let Some(index) = atom.variable_index(lhs) {
//                                 criteria.push(Criteria::new(
//                                     index,
//                                     *op,
//                                     CriteriaValue::Value(rhs.clone()),
//                                 ))
//                             }
//                         }
//                         (Term::Constant(lhs), op, Term::Variable(rhs)) => {
//                             if let Some(index) = atom.variable_index(rhs) {
//                                 criteria.push(Criteria::new(
//                                     index,
//                                     op.inverse(),
//                                     CriteriaValue::Value(lhs.clone()),
//                                 ));
//                             }
//                         }
//                         (Term::Variable(lhs), op, Term::Variable(rhs)) => {
//                             if let Some(lhs_index) = atom.variable_index(lhs) {
//                                 if let Some(rhs_index) = atom.variable_index(rhs) {
//                                     criteria.push(Criteria::new(
//                                         lhs_index,
//                                         *op,
//                                         CriteriaValue::Index(rhs_index),
//                                     ));
//                                 } else {
//                                     theta.push(comparison);
//                                 }
//                             }
//                         }
//                         _ => unreachable!(),
//                     }
//                 }
//             }
//             let atom_op = Self::compile_atom_with(atom, false, criteria)?;
//             println!("compile_rule > atom >> {}", atom_op);
//             ops.push(atom_op);
//         }
//
//         warn!(
//             "Found comparisons for theta join, which is not yet implemented: {:?}",
//             theta
//         );
//
//         let mut ops = ops.into_iter().rev();
//         let last = ops.next().unwrap();
//         let joined = ops.fold(last, |left, right| Join::natural(left, right).into());
//         println!("compile_rule > joined {:?}", joined);
//
//         // TODO: (ISSUE/rust-asdi/4) may need rework for disjunction.
//
//         let distinguished_terms = rule.distinguished_terms_in_order();
//         let joined = if distinguished_terms.len() < rule.variables().len() {
//             // TODO: (ISSUE/rust-asdi/12) Need to support constants in the final projection.
//             let joined = RelationalOp::from(Projection::new(
//                 distinguished_terms
//                     .iter()
//                     .filter_map(|t| t.as_variable())
//                     .map(|v| Attribute::labeled(v.clone()))
//                     .collect::<Vec<Attribute<Variable>>>(),
//                 joined,
//             ));
//             println!("compile_rule > joined {:?}", joined);
//             joined
//         } else {
//             joined
//         };
//         Ok(RelationalOp::Sink(RelationSink::new(
//             joined,
//             rule.head.get(0).unwrap().label_ref(),
//         )))
//     }
// }
//
// // ------------------------------------------------------------------------------------------------
//
// impl Selection {
//     pub fn is_match(&self, fact: &[Value]) -> Result<bool> {
//         for criteria in &self.criteria {
//             if !criteria.is_match(fact)? {
//                 return Ok(false);
//             }
//         }
//         Ok(true)
//     }
// }
//
// impl TryFrom<&Atom> for Selection {
//     type Error = Error;
//
//     fn try_from(value: &Atom) -> std::result::Result<Self, Self::Error> {
//         Ok(Self {
//             source: Box::new(RelationalOp::Relation(value.label_ref().into())),
//             criteria: value
//                 .iter()
//                 .enumerate()
//                 .filter_map(|(i, term)| term.as_constant().map(|c| (i, c)))
//                 .map(|(i, constant)| Criteria {
//                     index: i,
//                     op: ComparisonOperator::Equal,
//                     value: CriteriaValue::Value(constant.clone()),
//                 })
//                 .collect(),
//             negated: false,
//         })
//     }
// }
//
// impl TryFrom<&Rule> for Selection {
//     type Error = Error;
//
//     fn try_from(_value: &Rule) -> std::result::Result<Self, Self::Error> {
//         unimplemented!()
//     }
// }
//
// // ------------------------------------------------------------------------------------------------
//
// impl Criteria {
//     pub fn is_match(&self, fact: &[]) -> Result<bool> {
//         let lhs = fact
//             .get(self.index)
//             .ok_or_else(|| attribute_index_invalid(self.index))?;
//         let rhs = match &self.value {
//             ProjectedAttribute::Value(v) => v,
//             ProjectedAttribute::Index(i) => fact.get(*i).ok_or_else(|| attribute_index_invalid(*i))?,
//         };
//         if lhs.kind() != rhs.kind() {
//             Err(incompatible_types(
//                 lhs.kind().to_string(),
//                 rhs.kind().to_string(),
//             ))
//         } else {
//             Ok(match self.op {
//                 ComparisonOperator::Equal => lhs == rhs,
//                 ComparisonOperator::NotEqual => lhs != rhs,
//                 ComparisonOperator::LessThan => lhs < rhs,
//                 ComparisonOperator::LessThanOrEqual => lhs <= rhs,
//                 ComparisonOperator::GreaterThan => lhs > rhs,
//                 ComparisonOperator::GreaterThanOrEqual => lhs >= rhs,
//                 ComparisonOperator::StringMatch => {
//                     // TODO: cache regex
//                     let lhs = lhs.as_string().unwrap();
//                     let rhs = rhs.as_string().unwrap();
//                     let regex: Regex = Regex::new(rhs).unwrap();
//                     regex.is_match(lhs)
//                 }
//             })
//         }
//     }
// }
//
//
// impl TryFrom<&Atom> for Projection {
//     type Error = Error;
//
//     fn try_from(atom: &Atom) -> std::result::Result<Self, Self::Error> {
//         let projections: Vec<Attribute<Variable>> = atom
//             .iter()
//             .enumerate()
//             .filter(|(_, term)| !term.is_anonymous())
//             .map(|(i, term)| {
//                 let mut attribute = match term {
//                     Term::Variable(v) => Attribute::from(v.clone()),
//                     Term::Constant(v) => Attribute::typed(v.kind()),
//                     Term::Anonymous => unreachable!(),
//                 };
//                 attribute.set_index(i);
//                 attribute
//             })
//             .collect();
//
//         if projections.len() == atom.len() {
//             Ok(Self::all(RelationalOp::Relation(atom.label_ref().into())))
//         } else if projections.is_empty() {
//             Err(nullary_facts_not_allowed())
//         } else {
//             Ok(Self::new(
//                 projections,
//                 RelationalOp::Relation(atom.label_ref().into()),
//             ))
//         }
//     }
// }
//
// impl From<Projection> for Schema<Variable> {
//     fn from(p: Projection) -> Self {
//         Self::from(
//             p.attributes
//                 .into_iter()
//                 .collect::<Vec<Attribute<Variable>>>(),
//         )
//     }
// }

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
