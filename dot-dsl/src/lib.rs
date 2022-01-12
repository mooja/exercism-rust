
pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    type Attributes = HashMap<String, String>;

    pub mod graph_items {
        pub mod edge {
            use super::super::Attributes;
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Edge {
                pub a: String,
                pub b: String,
                pub attrs: Attributes
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        a: String::from(a),
                        b: String::from(b),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs<'a >(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, val) in attrs.iter() {
                        self.attrs.insert(String::from(*key), String::from(*val));
                    }
                    self
                } 
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                pub attrs: crate::graph::Attributes,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: String::from(name),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self{
                    for (prop, val) in attrs {
                        self.attrs.insert(String::from(*prop), String::from(*val));
                    }
                    self
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    match self.attrs.get(name) {
                        None => None,
                        Some(s) => Some(&s[..])
                    }
                }
            }
        }
    }

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: crate::graph::Attributes
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new()
            }
        }

        pub fn with_nodes<'a >(mut self, nodes: &'a Vec<Node>) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            self
        } 

        pub fn with_edges<'a >(mut self, edges: &'a Vec<Edge>) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            self
        } 

        pub fn with_attrs<'a >(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, val) in attrs.iter() {
                self.attrs.insert(String::from(*key), String::from(*val));
            }
            self
        } 

        pub fn get_node(&self, name: &str) -> Result<&Node, &str> {
            match self.nodes.iter().find(|node| node.name == name) {
                None => Err("node must be stored"),
                Some(n) => Ok(n)
            }
        }
    }
}
