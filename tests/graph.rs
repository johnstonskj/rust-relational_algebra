#[cfg(all(feature = "graphviz", test))]
mod tests {
    use relational_algebra::{ast::RelationalOp, graph::relational_to_graphviz, Name};
    use simple_dot::writer::write_graph_to_string;

    #[test]
    fn test_generate_dot() {
        let expr = RelationalOp::natural_join(
            Name::new_unchecked("people"),
            RelationalOp::union(Name::new_unchecked("visits"), Name::new_unchecked("places")),
        );

        println!("{:#?}", expr);

        let graph = relational_to_graphviz(&expr).unwrap();

        println!("{}", write_graph_to_string(&graph).unwrap());
    }
}
