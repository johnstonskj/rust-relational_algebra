use std::collections::HashMap;

use relational_algebra::{
    ast::{Attribute, Expression, ExpressionList, RelationalOp, Term},
    data::Value,
    Name,
};

#[test]
fn test_term_atom() {
    assert_eq!(format!("{}", Term::equals(0, 1)), String::from("0=1"));
    assert_eq!(
        format!("{}", Term::not_equals(Name::new_unchecked("a"), 1)),
        String::from("a≠1")
    );
    assert_eq!(
        format!(
            "{}",
            Term::less_than(Name::new_unchecked("a"), Name::new_unchecked("b"))
        ),
        String::from("a<b")
    );
    assert_eq!(
        format!("{}", Term::string_match(0, Value::from("foo*"))),
        String::from("0~\"foo*\"")
    );
    assert_eq!(
        format!("{}", Term::string_not_match(0, Value::from("foo*"))),
        String::from("0≁\"foo*\"")
    );
}

#[test]
fn test_term_nested_and() {
    let ast = Term::and(
        Name::new_unchecked("a"),
        Term::and(Name::new_unchecked("b"), Name::new_unchecked("c")),
    );
    println!("{:#?}", ast);
    assert_eq!(format!("{}", ast), String::from("?a ∧ ?b ∧ ?c"));
}

#[test]
fn test_relation_only() {
    let ast = RelationalOp::relation_unchecked("relation");
    assert_eq!(format!("{}", ast), String::from("relation"));
}

#[test]
fn test_set_operation_only() {
    let ast = RelationalOp::union(Name::new_unchecked("left"), Name::new_unchecked("right"));
    assert_eq!(format!("{}", ast), String::from("left ∪ right"));
}

#[test]
fn test_selection_only() {
    let ast = RelationalOp::select(Term::equals(0, 1), Name::new_unchecked("relation"));
    assert_eq!(format!("{}", ast), String::from("σ[0=1]relation"));
}

#[test]
fn test_projection_only() {
    let ast = RelationalOp::project(
        vec![2.into(), Name::new_unchecked("a").into(), 0.into()],
        Name::new_unchecked("relation"),
    );
    assert_eq!(format!("{}", ast), String::from("π[2, a, 0]relation"));
}

#[test]
fn test_rename_only() {
    let ast = RelationalOp::rename(
        [(0.into(), Name::new_unchecked("a"))]
            .into_iter()
            .collect::<HashMap<Attribute, Name>>(),
        Name::new_unchecked("relation"),
    )
    .unwrap();
    assert_eq!(format!("{}", ast), String::from("ρ[a]relation"));
}

#[test]
fn test_natural_join_only() {
    let ast = RelationalOp::natural_join(Name::new_unchecked("left"), Name::new_unchecked("right"));
    assert_eq!(format!("{}", ast), String::from("left ⨝ right"));
}

#[test]
fn test_theta_join_only() {
    let ast = RelationalOp::theta_join(
        Name::new_unchecked("left"),
        Term::equals(0, 1),
        Name::new_unchecked("right"),
    );
    assert_eq!(format!("{}", ast), String::from("left ⨝[0=1] right"));
}

#[test]
fn test_unnamed_expression() {
    let ast: ExpressionList = Expression::new(RelationalOp::union(
        Name::new_unchecked("left"),
        Name::new_unchecked("right"),
    ))
    .into();
    assert_eq!(format!("{}", ast), String::from("left ∪ right;\n"));
}

#[test]
fn test_named_expression() {
    let ast: ExpressionList = Expression::named(
        Name::new_unchecked("A"),
        RelationalOp::union(Name::new_unchecked("left"), Name::new_unchecked("right")),
    )
    .into();
    assert_eq!(format!("{}", ast), String::from("A ≔ left ∪ right;\n"));
}
