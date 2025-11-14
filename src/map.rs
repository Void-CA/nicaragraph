use std::collections::HashMap;

use crate::models::{GraphJson, Node};
use ordered_float::OrderedFloat;
use suma_core::core::data_structures::graphs::{UndirectedWeightedGraph};

pub fn build_graph_from_json(graph_json: &GraphJson) -> UndirectedWeightedGraph<Node, OrderedFloat<f64>> {
    let mut graph = UndirectedWeightedGraph::<Node, OrderedFloat<f64>>::new();
    let mut node_index: HashMap<String, usize> = HashMap::new();

    // Agregar nodos
    for node in graph_json.nodes.clone() {
        let idx = graph.base.add_node(node.clone());
        node_index.insert(node.id.clone(), idx);
    }

    // Agregar aristas con pesos Euclidianos
    for node in &graph_json.nodes {
        let from_idx = node_index[&node.id];
        for neighbor_id in &node.neighbors {
            if let Some(&to_idx) = node_index.get(neighbor_id) {
                let node_b = &graph_json.nodes[to_idx];
                let dx = node.lon - node_b.lon;
                let dy = node.lat - node_b.lat;
                let weight = OrderedFloat((dx*dx + dy*dy).sqrt());
                graph.add_weighted_edge(from_idx, to_idx, weight);
            }
        }
    }

    graph
}
