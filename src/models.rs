use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub dept: String,
    pub lon: f64,
    pub lat: f64,
    pub neighbors: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GraphJson {
    pub nodes: Vec<Node>,
}
