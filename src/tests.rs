#[cfg(test)]
mod tests {
    use crate::map::build_graph_from_json;
    use crate::models::GraphJson;
    use ordered_float::OrderedFloat;
    use suma_core::core::data_structures::graphs::{GraphBase, UndirectedWeightedGraph, WeightedGraph};

    #[test]
    fn test_build_graph_from_json() {
        let json_data = r#"
        {
            "nodes": [
                {"id": "1", "name": "Node1", "dept": "Dept1", "lon": 0.0, "lat": 0.0, "neighbors": ["2"]},
                {"id": "2", "name": "Node2", "dept": "Dept2", "lon": 3.0, "lat": 4.0, "neighbors": ["1"]}
            ]
        }
        "#;

        // Parse JSON
        let graph_json: GraphJson = serde_json::from_str(json_data).unwrap();

        // Build graph
        let graph = build_graph_from_json(&graph_json);

        // Validate graph structure
        assert_eq!(graph.base.node_count(), 2, "Should register 2 nodes");
        assert_eq!(graph.base.edge_count(), 2, "Should have exactly 2 undirected edges");

        // Nodes are inserted in order: "1" -> 0, "2" -> 1
        let id_1 = 0usize;
        let id_2 = 1usize;

        // Test computed distance
        let weight = graph.edge_weight(id_1, id_2).expect("Edge missing between 1 and 2");

        assert_eq!(weight, OrderedFloat(5.0), "Distance must be 5.0 (3-4-5 triangle)");
    }
}
