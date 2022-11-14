/*!
This module allows for the generation of a [GraphViz](https://graphviz.org/) DOT representation of an expression.

[DOT](https://graphviz.org/doc/info/lang.html)

 */

use crate::{
    ast::{
        Attribute, Group, Join, Order, ProjectedAttribute, Projection, RelationalOp, Rename,
        Selection, SetOperation,
    },
    error::Result,
    Name,
};
use simple_dot::{
    attributes::{GraphAttributes, LabelString, NodeAttributes, NodeStyles, Styled},
    graph::Graph,
    Edge, Identified, Identifier as DotId, Node, RootGraph,
};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn relational_to_graphviz(op: &RelationalOp) -> Result<RootGraph> {
    let progress = relational_to_node(op)?;

    Ok(RootGraph::anonymous(false, true)
        .set_attributes(GraphAttributes::default().root(progress.target.to_string()))
        .add_nodes(progress.nodes)
        .add_edges(progress.edges))
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

struct Progress {
    target: DotId,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn relational_to_node(op: &RelationalOp) -> Result<Progress> {
    Ok(match op {
        RelationalOp::Relation(v) => relation_to_node(v)?,
        RelationalOp::SetOperation(v) => set_operation_to_node(v)?,
        RelationalOp::Selection(v) => selection_to_node(v)?,
        RelationalOp::Projection(v) => projection_to_node(v)?,
        RelationalOp::Rename(v) => rename_to_node(v)?,
        RelationalOp::Order(v) => order_to_node(v)?,
        RelationalOp::Group(v) => group_to_node(v)?,
        RelationalOp::Join(v) => join_to_node(v)?,
    })
}

fn relation_to_node(relation: &Name) -> Result<Progress> {
    let node = Node::new(DotId::new_node()).set_attributes(
        NodeAttributes::default()
            .style(vec![NodeStyles::Filled])
            .label(LabelString::from_str(&relation.to_string()).unwrap()),
    );
    Ok(Progress {
        target: node.id().clone(),
        nodes: vec![node],
        edges: Default::default(),
    })
}

fn set_operation_to_node(set_operation: &SetOperation) -> Result<Progress> {
    let lhs = relational_to_node(set_operation.lhs())?;
    let rhs = relational_to_node(set_operation.rhs())?;

    let node_id = DotId::new_node();
    let mut nodes = vec![Node::new(node_id.clone()).set_attributes(
        NodeAttributes::default()
            .label(LabelString::from_str(&format!("{}", set_operation.operator())).unwrap()),
    )];
    nodes.extend(lhs.nodes);
    nodes.extend(rhs.nodes);

    let mut edges = vec![
        Edge::new(node_id.clone(), lhs.target),
        Edge::new(node_id.clone(), rhs.target),
    ];
    edges.extend(lhs.edges);
    edges.extend(rhs.edges);

    Ok(Progress {
        target: node_id,
        nodes,
        edges,
    })
}

fn selection_to_node(selection: &Selection) -> Result<Progress> {
    let rhs = relational_to_node(selection.rhs())?;

    let node_id = DotId::new_node();
    let mut nodes = vec![Node::new(node_id.clone()).set_attributes(
        NodeAttributes::default()
            .label(LabelString::from_str(&format!("σ\n{}", selection.criteria())).unwrap()),
    )];
    nodes.extend(rhs.nodes);

    let mut edges = vec![Edge::new(node_id.clone(), rhs.target)];
    edges.extend(rhs.edges);

    Ok(Progress {
        target: node_id,
        nodes,
        edges,
    })
}

fn projection_to_node(projection: &Projection) -> Result<Progress> {
    let rhs = relational_to_node(projection.rhs())?;

    let node_id = DotId::new_node();
    let mut nodes = vec![Node::new(node_id.clone()).set_attributes(
        NodeAttributes::default().label(
            LabelString::from_str(&format!(
                "Π\n{}",
                projection
                    .attributes()
                    .map(ProjectedAttribute::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .unwrap(),
        ),
    )];
    nodes.extend(rhs.nodes);

    let mut edges = vec![Edge::new(node_id.clone(), rhs.target)];
    edges.extend(rhs.edges);

    Ok(Progress {
        target: node_id,
        nodes,
        edges,
    })
}

fn rename_to_node(rename: &Rename) -> Result<Progress> {
    let rhs = relational_to_node(rename.rhs())?;

    let node_id = DotId::new_node();
    let mut nodes = vec![Node::new(node_id.clone()).set_attributes(
        NodeAttributes::default().label(
            LabelString::from_str(&format!(
                "ρ\n{}",
                rename
                    .renames()
                    .map(|(a, i)| format!("{}/{}", a, i))
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .unwrap(),
        ),
    )];
    nodes.extend(rhs.nodes);

    let mut edges = vec![Edge::new(node_id.clone(), rhs.target)];
    edges.extend(rhs.edges);

    Ok(Progress {
        target: node_id,
        nodes,
        edges,
    })
}

fn order_to_node(order: &Order) -> Result<Progress> {
    let rhs = relational_to_node(order.rhs())?;

    let node_id = DotId::new_node();
    let mut nodes = vec![Node::new(node_id.clone()).set_attributes(
        NodeAttributes::default().label(
            LabelString::from_str(&format!(
                "τ\n{}",
                order
                    .attributes()
                    .map(Attribute::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .unwrap(),
        ),
    )];
    nodes.extend(rhs.nodes);

    let mut edges = vec![Edge::new(node_id.clone(), rhs.target)];
    edges.extend(rhs.edges);

    Ok(Progress {
        target: node_id,
        nodes,
        edges,
    })
}

fn group_to_node(group: &Group) -> Result<Progress> {
    let rhs = relational_to_node(group.rhs())?;

    let node_id = DotId::new_node();
    let mut nodes = vec![Node::new(node_id.clone()).set_attributes(
        NodeAttributes::default().label(
            LabelString::from_str(&format!(
                "γ\n{}",
                group
                    .attributes()
                    .map(Attribute::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .unwrap(),
        ),
    )];
    nodes.extend(rhs.nodes);

    let mut edges = vec![Edge::new(node_id.clone(), rhs.target)];
    edges.extend(rhs.edges);

    Ok(Progress {
        target: node_id,
        nodes,
        edges,
    })
}

fn join_to_node(join: &Join) -> Result<Progress> {
    let (lhs, criteria, rhs) = match join {
        Join::Natural(j) => (j.lhs(), None, j.rhs()),
        Join::Theta(j) => (j.lhs(), Some(j.criteria()), j.rhs()),
    };

    let lhs = relational_to_node(lhs)?;
    let rhs = relational_to_node(rhs)?;

    let label_string = if let Some(criteria) = criteria {
        LabelString::from_str(&format!("⨝\n{}", criteria)).unwrap()
    } else {
        LabelString::from_str("⨝").unwrap()
    };

    let node_id = DotId::new_node();
    let mut nodes =
        vec![Node::new(node_id.clone())
            .set_attributes(NodeAttributes::default().label(label_string))];
    nodes.extend(lhs.nodes);
    nodes.extend(rhs.nodes);

    let mut edges = vec![
        Edge::new(node_id.clone(), lhs.target),
        Edge::new(node_id.clone(), rhs.target),
    ];
    edges.extend(lhs.edges);
    edges.extend(rhs.edges);

    Ok(Progress {
        target: node_id,
        nodes,
        edges,
    })
}

// fn assignment_to_node(assignment: &Assignment) -> Result<Progress> {
//     let rhs = relational_to_node(assignment.rhs())?;
//
//     let node_id = DotId::new_node();
//     let mut nodes = vec![Node::new(node_id.clone()).set_attributes(
//         NodeAttributes::default()
//             .style(vec![NodeStyles::Filled])
//             .label(LabelString::from_str(&format!("α\n{}", assignment.name())).unwrap()),
//     )];
//     nodes.extend(rhs.nodes);
//
//     let mut edges = vec![Edge::new(node_id.clone(), rhs.target)];
//     edges.extend(rhs.edges);
//
//     Ok(Progress {
//         target: node_id,
//         nodes,
//         edges,
//     })
// }
