use std::collections::HashMap;

#[derive(Debug)]
enum NodeType {
    IN,
    OUT,
    ACTIVATION,
}

#[derive(Debug)]
pub struct Node {
    pub unique_id: u32,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
    weight: f32,
    enabled: bool,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<u32, Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, unique_id: u32, node_type: NodeType) {
        // check if the vertex already exists
        if self.nodes.contains_key(&unique_id) {
            return;
        } else {
            self.nodes.insert(
                unique_id,
                Node {
                    unique_id,
                    node_type,
                },
            );
        }
    }

    pub fn add_edge(&mut self, from: u32, to: u32, weight: f32, enabled: bool) {
        // check if the edge already exists
        if self
            .edges
            .iter()
            .any(|edge| edge.from == from && edge.to == to)
        {
            return;
        } else {
            self.edges.push(Edge {
                from,
                to,
                weight,
                enabled,
            });
        }
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        for (unique_id, node) in &self.nodes {
            println!("{:?}", node);
        }

        for edge in &self.edges {
            println!("{:?}", edge);
        }
    }

}
