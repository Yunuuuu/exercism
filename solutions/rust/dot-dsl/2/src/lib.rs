pub mod graph {
    use std::collections::HashMap;
    type Attrs = HashMap<String, String>;

    pub mod graph_items {
        // define graph node
        pub mod node {
            use crate::graph::Attrs;
            #[derive(Debug, Default, PartialEq, Eq, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: Attrs,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &**s)
                }
            }
        }
        // define graph edge
        pub mod edge {
            use crate::graph::Attrs;
            #[derive(Debug, Default, PartialEq, Eq, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: Attrs,
            }
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &**s)
                }
            }
        }
    }
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: Attrs,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: Attrs::new(),
            }
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }
        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }
}
