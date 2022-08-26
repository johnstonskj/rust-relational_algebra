use relational_algebra::{ast::RelationalOp, graph::relational_to_graphviz, Identifier};
use simple_dot::writer::write_graph_to_string;

#[test]
fn test_generate_dot() {
    let expr = RelationalOp::assign(
        Identifier::new_unchecked("who_and_where"),
        RelationalOp::natural_join(
            Identifier::new_unchecked("people"),
            RelationalOp::union(
                Identifier::new_unchecked("visits"),
                Identifier::new_unchecked("places"),
            ),
        ),
    );

    println!("{:#?}", expr);

    let graph = relational_to_graphviz(&expr).unwrap();

    println!("{}", write_graph_to_string(&graph).unwrap());
}
