use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use suma_core::core::data_structures::graphs::{
    GraphBase, WeightedGraph, algorithms::{a_star::*, djikstra::*, search::*}
};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

mod map;
mod models;
use crate::models::GraphJson;

#[wasm_bindgen]
pub fn run_astar(start_id: String, goal_id: String, graph_json: JsValue) -> JsValue {
    let graph_json: GraphJson = serde_wasm_bindgen::from_value(graph_json).unwrap();
    let graph = map::build_graph_from_json(&graph_json);

    let mut id_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in graph_json.nodes.iter().enumerate() {
        id_to_index.insert(node.id.clone(), i);
    }
    let start = id_to_index[&start_id];
    let goal  = id_to_index[&goal_id];

    let heuristic = |a: &usize, b: &usize| {
        let node_a = graph.base.node_data(*a).unwrap();
        let node_b = graph.base.node_data(*b).unwrap();
        OrderedFloat::from(((node_a.lon - node_b.lon).powi(2) + (node_a.lat - node_b.lat).powi(2)).sqrt())
    };

    let (path, cost, visited) = a_star_traversal(&graph, start, goal, heuristic).unwrap();

    let path_ids: Vec<String> = path.iter().map(|i| graph_json.nodes[*i].id.clone()).collect();
    let visited_ids: Vec<String> = visited.iter().map(|i| graph_json.nodes[*i].id.clone()).collect();

    let output = serde_wasm_bindgen::to_value(&serde_json::json!({
        "path": path_ids,
        "visited": visited_ids,
        "cost": cost
    })).unwrap();

    output
}

#[wasm_bindgen]
pub fn run_bfs(start_id: String, goal_id: String, graph_json: JsValue) -> JsValue {
    let graph_json: GraphJson = serde_wasm_bindgen::from_value(graph_json).unwrap();
    let graph = map::build_graph_from_json(&graph_json);

    let mut id_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in graph_json.nodes.iter().enumerate() {
        id_to_index.insert(node.id.clone(), i);
    }
    
    let start = id_to_index[&start_id];
    let goal = id_to_index[&goal_id];

    // ✅ USAR la función bfs que ya tienes implementada
    if let Some(path_indices) = bfs(&graph, start, goal) {
        let path_ids: Vec<String> = path_indices.iter()
            .map(|&i| graph_json.nodes[i].id.clone())
            .collect();
        
        // Para visited, usar bfs_traversal
        let (visited_set, traversal_order) = bfs_traversal(&graph, start);
        let visited_ids: Vec<String> = traversal_order.iter()
            .map(|&i| graph_json.nodes[i].id.clone())
            .collect();

        serde_wasm_bindgen::to_value(&serde_json::json!({
            "path": path_ids,
            "visited": visited_ids,
            "cost": path_ids.len() as f64 - 1.0 // Costo = longitud del camino
        })).unwrap()
    } else {
        // Si no hay camino, devolver visited nodes igualmente
        let (visited_set, traversal_order) = bfs_traversal(&graph, start);
        let visited_ids: Vec<String> = traversal_order.iter()
            .map(|&i| graph_json.nodes[i].id.clone())
            .collect();

        serde_wasm_bindgen::to_value(&serde_json::json!({
            "path": [],
            "visited": visited_ids,
            "cost": null
        })).unwrap()
    }
}

#[wasm_bindgen]
pub fn run_dfs(start_id: String, goal_id: String, graph_json: JsValue) -> JsValue {
    let graph_json: GraphJson = serde_wasm_bindgen::from_value(graph_json).unwrap();
    let graph = map::build_graph_from_json(&graph_json);

    let mut id_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in graph_json.nodes.iter().enumerate() {
        id_to_index.insert(node.id.clone(), i);
    }
    
    let start = id_to_index[&start_id];
    let goal = id_to_index[&goal_id];

    // ✅ USAR la función dfs que ya tienes implementada
    if let Some(path_indices) = dfs(&graph, start, goal) {
        let path_ids: Vec<String> = path_indices.iter()
            .map(|&i| graph_json.nodes[i].id.clone())
            .collect();
        
        // Para visited, usar dfs_traversal
        let (visited_set, traversal_order) = dfs_traversal(&graph, start);
        let visited_ids: Vec<String> = traversal_order.iter()
            .map(|&i| graph_json.nodes[i].id.clone())
            .collect();

        serde_wasm_bindgen::to_value(&serde_json::json!({
            "path": path_ids,
            "visited": visited_ids,
            "cost": path_ids.len() as f64 - 1.0
        })).unwrap()
    } else {
        let (visited_set, traversal_order) = dfs_traversal(&graph, start);
        let visited_ids: Vec<String> = traversal_order.iter()
            .map(|&i| graph_json.nodes[i].id.clone())
            .collect();

        serde_wasm_bindgen::to_value(&serde_json::json!({
            "path": [],
            "visited": visited_ids,
            "cost": null
        })).unwrap()
    }
}

#[wasm_bindgen]
pub fn run_dijkstra(start_id: String, goal_id: String, graph_json: JsValue) -> JsValue {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"🔧 Starting Dijkstra...".into());

    // ✅ Parse seguro del JSON
    let graph_json: GraphJson = match serde_wasm_bindgen::from_value(graph_json) {
        Ok(json) => json,
        Err(e) => {
            web_sys::console::error_1(&format!("❌ JSON parse error: {:?}", e).into());
            return serde_wasm_bindgen::to_value(&serde_json::json!({
                "path": [],
                "visited": [],
                "cost": null,
                "error": "Invalid graph data"
            })).unwrap();
        }
    };

    web_sys::console::log_1(&format!("📊 Graph has {} nodes", graph_json.nodes.len()).into());

    // ✅ Construir mapa de IDs de manera segura
    let mut id_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in graph_json.nodes.iter().enumerate() {
        id_to_index.insert(node.id.clone(), i);
    }

    // ✅ Validar IDs
    let start_index = match id_to_index.get(&start_id) {
        Some(&idx) => idx,
        None => {
            web_sys::console::error_1(&format!("❌ Start node not found: {}", start_id).into());
            return serde_wasm_bindgen::to_value(&serde_json::json!({
                "path": [],
                "visited": [],
                "cost": null,
                "error": format!("Start node not found: {}", start_id)
            })).unwrap();
        }
    };

    let goal_index = match id_to_index.get(&goal_id) {
        Some(&idx) => idx,
        None => {
            web_sys::console::error_1(&format!("❌ Goal node not found: {}", goal_id).into());
            return serde_wasm_bindgen::to_value(&serde_json::json!({
                "path": [],
                "visited": [],
                "cost": null,
                "error": format!("Goal node not found: {}", goal_id)
            })).unwrap();
        }
    };

    web_sys::console::log_1(&format!("🎯 Dijkstra indices: {} -> {}", start_index, goal_index).into());

    // ✅ Construir grafo
    let graph = map::build_graph_from_json(&graph_json);
    
    // ✅ Ejecutar Dijkstra
    match dijkstra_path(&graph, start_index, goal_index) {
        Some((path_indices, cost)) => {
            web_sys::console::log_1(&format!("✅ Dijkstra found path with cost: {:?}", cost).into());
            
            let path_ids: Vec<String> = path_indices.iter()
                .map(|&i| graph_json.nodes[i].id.clone())
                .collect();

            // Obtener nodos visitados
            let distances = dijkstra_algorithm(&graph, start_index);
            let visited_ids: Vec<String> = distances.keys()
                .map(|&i| graph_json.nodes[i].id.clone())
                .collect();

            web_sys::console::log_1(&format!("📈 Path length: {}, Visited: {}", path_ids.len(), visited_ids.len()).into());

            serde_wasm_bindgen::to_value(&serde_json::json!({
                "path": path_ids,
                "visited": visited_ids,
                "cost": cost.into_inner()
            })).unwrap()
        },
        None => {
            web_sys::console::log_1(&"⚠️ Dijkstra: No path found".into());
            
            // Devolver al menos algunos nodos visitados
            let distances = dijkstra_algorithm(&graph, start_index);
            let visited_ids: Vec<String> = distances.keys()
                .map(|&i| graph_json.nodes[i].id.clone())
                .collect();

            serde_wasm_bindgen::to_value(&serde_json::json!({
                "path": [],
                "visited": visited_ids,
                "cost": null
            })).unwrap()
        }
    }
}