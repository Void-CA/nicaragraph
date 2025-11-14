use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use suma_core::core::data_structures::graphs::{
    GraphBase, algorithms::{a_star::*}
};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

mod map;
mod models;
use crate::models::GraphJson;

#[wasm_bindgen]
pub fn run_astar(start_id: String, goal_id: String, graph_json: JsValue) -> JsValue {
    // Convertimos JsValue -> GraphJson
    let graph_json: GraphJson = serde_wasm_bindgen::from_value(graph_json).unwrap();

    // Construimos el grafo interno
    let graph = map::build_graph_from_json(&graph_json);

    // Creamos un map de ID string -> índice usize
    let mut id_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in graph_json.nodes.iter().enumerate() {
        id_to_index.insert(node.id.clone(), i);
    }

    let start = id_to_index[&start_id];
    let goal  = id_to_index[&goal_id];

    // Heurística Euclidiana usando índices
    let heuristic = |a: &usize, b: &usize| {
        let node_a = graph.base.node_data(*a).unwrap();
        let node_b = graph.base.node_data(*b).unwrap();
        OrderedFloat::from(((node_a.lon - node_b.lon).powi(2) + (node_a.lat - node_b.lat).powi(2)).sqrt())
    };

    // Ejecutamos A*
    let result = a_star_traversal(&graph, start, goal, heuristic);

    // Convertimos de vuelta los índices a IDs string para el frontend
    let result_with_ids = result.map(|(path, cost, visited)| {
        let path_ids: Vec<String> = path.iter().map(|i| graph_json.nodes[*i].id.clone()).collect();
        let visited_ids: Vec<String> = visited.iter().map(|i| graph_json.nodes[*i].id.clone()).collect();
        (path_ids, cost, visited_ids)
    });

    serde_wasm_bindgen::to_value(&result_with_ids).unwrap()
}
