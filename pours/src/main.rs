#![allow(dead_code, unused_imports, unused_variables)]
use petgraph::Graph;
use petgraph::algo::*;
use petgraph::visit::{GraphBase, NodeRef};
use petgraph::graph::NodeIndex;

#[derive(Default, Debug)]
struct Task {
    name: String,
    duration: i32,
}

fn main() {
    let mut deps = Graph::<Task, Task>::new();

    let c = deps.add_node(Task { name: "c".to_string(), duration: 0 });
    let a = deps.add_node(Task { name: "a".to_string(), duration: 0 });
    let b = deps.add_node(Task { name: "b".to_string(), duration: 0 });


    deps.extend_with_edges(&[
        (a, b), (a, c),
    ]);

    // let sorted = petgraph::algo::toposort(&deps,None).unwrap();
    // deps.into_nodes_edges().0.into_iter().for_each(|n| println!("{:?}", n.weight));

    let sorted: Vec<NodeIndex> = petgraph::algo::toposort(&deps,None).unwrap();
    // sorted.into_iter().for_each(|i| println!("{:?}", deps.node_weight(i).unwrap()));

    // deps.node_weight_mut(sorted[0].name).unwrap() = &mut Task { name: "new a".to_string(), duration: 0 };

    println!("{:?}",deps.node_weight(sorted[0]))

    // match toposort(&deps, None) {
    //     Ok(order) => {
    //         for i in order {
    //             println!("{:?}, ", i);
    //             // deps.node_weight(i).map(|task| {
    //             //     println!("{:?}, ", task);
    //             //     // weight
    //             // });
    //         }
    //     },
    //     Err(_) => {}
    // }
}
