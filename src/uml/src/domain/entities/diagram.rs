#[derive(Debug, PartialEq, Default)]
pub struct Diagram {
    pub title: Option<String>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Diagram {
    pub fn new(title: Option<String>, nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Self {
            title,
            nodes,
            edges,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub id: String,
    pub r#type: NodeType,
}

impl Node {
    pub fn new(id: String, r#type: NodeType) -> Self {
        Self { id, r#type }
    }
}

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub from_id: String,
    pub to_id: String,
    pub style_from: EdgeStyle,
    pub style_to: EdgeStyle,
    pub label: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    // Name
    Component(String),
}

#[derive(Debug, PartialEq)]
pub enum EdgeStyle {
    Solid,     // ---
    Arrow,     // -->
    OpenArrow, // --<
}
