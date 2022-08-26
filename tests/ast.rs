use std::collections::HashMap;

use relational_algebra::{
    ast::{Attribute, RelationalOp, Term},
    data::Value,
    Identifier,
};

#[test]
fn test_term_atom() {
    assert_eq!(format!("{}", Term::equals(0, 1)), String::from("0=1"));
    assert_eq!(
        format!("{}", Term::not_equals(Identifier::new_unchecked("a"), 1)),
        String::from("a≠1")
    );
    assert_eq!(
        format!(
            "{}",
            Term::less_than(
                Identifier::new_unchecked("a"),
                Identifier::new_unchecked("b")
            )
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
        Identifier::new_unchecked("a"),
        Term::and(
            Identifier::new_unchecked("b"),
            Identifier::new_unchecked("c"),
        ),
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
    let ast = RelationalOp::union(
        Identifier::new_unchecked("left"),
        Identifier::new_unchecked("right"),
    );
    assert_eq!(format!("{}", ast), String::from("left ∪ right"));
}

#[test]
fn test_selection_only() {
    let ast = RelationalOp::select(Term::equals(0, 1), Identifier::new_unchecked("relation"));
    assert_eq!(format!("{}", ast), String::from("σ[0=1]relation"));
}

#[test]
fn test_projection_only() {
    let ast = RelationalOp::project(
        vec![2.into(), Identifier::new_unchecked("a").into(), 0.into()],
        Identifier::new_unchecked("relation"),
    );
    assert_eq!(format!("{}", ast), String::from("Π[2, a, 0]relation"));
}

#[test]
fn test_rename_only() {
    let ast = RelationalOp::rename(
        [(0.into(), Identifier::new_unchecked("a"))]
            .into_iter()
            .collect::<HashMap<Attribute, Identifier>>(),
        Identifier::new_unchecked("relation"),
    )
    .unwrap();
    assert_eq!(format!("{}", ast), String::from("ρ[a]relation"));
}

#[test]
fn test_natural_join_only() {
    let ast = RelationalOp::natural_join(
        Identifier::new_unchecked("left"),
        Identifier::new_unchecked("right"),
    );
    assert_eq!(format!("{}", ast), String::from("left ⨝ right"));
}

#[test]
fn test_theta_join_only() {
    let ast = RelationalOp::theta_join(
        Identifier::new_unchecked("left"),
        Term::equals(0, 1),
        Identifier::new_unchecked("right"),
    );
    assert_eq!(format!("{}", ast), String::from("left ⨝[0=1] right"));
}

#[test]
fn test_assignment_only() {
    let ast = RelationalOp::assign(
        Identifier::new_unchecked("new_name"),
        Identifier::new_unchecked("old_relation"),
    );
    assert_eq!(format!("{}", ast), String::from("α[new_name]old_relation"));
}
