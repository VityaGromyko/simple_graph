#[cfg(test)]
mod tests {
    use crate::Graph;
    use std::collections::HashMap;

    #[test]
    fn test_add_vertex() {
        let mut graph: Graph<&str, &str, &str> = Graph::new();
        graph.add_vertex("A", "node");
        let vertices = HashMap::from([("A", "node")]);
        assert_eq!(graph.vertices, vertices);
        assert_eq!(graph.edges, HashMap::new());
    }

    #[test]
    fn test_remove_vertex() {
        let mut graph: Graph<&str, &str, &str> = Graph::new();
        graph.add_vertex("A", "node");
        graph.remove_vertex("A");
        assert_eq!(graph.vertices, HashMap::new());
        assert_eq!(graph.edges, HashMap::new());
    }

    #[test]
    fn test_add_edge() {
        let mut graph: Graph<&str, &str, &str> = Graph::new();
        graph.add_vertex("A", "node");
        graph.add_vertex("B", "node");
        graph.add_edge("A", "B", "edge");
        let vertices = HashMap::from([("A", "node"), ("B", "node")]);
        let adjacency = HashMap::from([("A", vec![("B", "edge")])]);
        assert_eq!(graph.vertices, vertices);
        assert_eq!(graph.edges, adjacency);
    }

    #[test]
    fn test_remove_edge() {
        let mut graph: Graph<&str, &str, &str> = Graph::new();
        graph.add_vertex("A", "node");
        graph.add_vertex("B", "node");
        graph.add_edge("A", "B", "edge");
        graph.remove_edge("A", "B");
        let vertices = HashMap::from([("A", "node"), ("B", "node")]);
        assert_eq!(graph.vertices, vertices);
        assert_eq!(graph.edges, HashMap::from([("A", vec![])]));
    }

    #[test]
    fn test_bfs() {
        let mut graph: Graph<&str, &str, &str> = Graph::new();
        graph.add_vertex("A", "node");
        graph.add_vertex("B", "node");
        graph.add_vertex("C", "node");
        graph.add_vertex("D", "node");
        graph.add_vertex("E", "node");
        graph.add_edge("A", "B", "edge");
        graph.add_edge("B", "C", "edge");
        graph.add_edge("B", "D", "edge");
        graph.add_edge("C", "B", "edge");
        graph.add_edge("C", "D", "edge");
        graph.add_edge("C", "E", "edge");
        graph.add_edge("D", "B", "edge");
        graph.add_edge("D", "C", "edge");
        assert!(graph.bfs("A", "E"));
        assert!(graph.bfs("A", "D"));
        assert!(!graph.bfs("E", "A"));
        assert!(!graph.bfs("E", "Uh no"));
    }

    #[test]
    fn test_serialize() {
        let mut graph: Graph<&str, &str, &str> = Graph::new();
        graph.add_vertex("A", "node");
        graph.add_vertex("B", "node");
        graph.add_vertex("C", "node");
        graph.add_edge("A", "B", "edge");
        graph.add_edge("B", "C", "edge");
        let graph_string = graph.serialize();
        let result_string = "\"A\" \"node\"\n\"B\" \"node\"\n\"C\" \"node\"\n#\n\"A\" \"B\" \"edge\"\n\"B\" \"C\" \"edge\"\n";
        let mut graph_vec = graph_string.split("\n").collect::<Vec<&str>>();
        let mut result_vec = result_string.split("\n").collect::<Vec<&str>>();
        graph_vec.sort();
        result_vec.sort();
        assert_eq!(graph_vec, result_vec);
    }

    #[test]
    fn test_from() {
        let mut graph = Graph::new();
        graph.add_vertex(1, "ha ha".to_string());
        graph.add_vertex(2, "haha".to_string());
        graph.add_edge(1, 2, 2.2);
        let graph_from_str: Graph<i32, String, f64> = Graph::from("2 \"haha\"\n1 \"ha ha\"\n#\n1 2 2.2\n");
        assert_eq!(graph.vertices, graph_from_str.vertices);
        assert_eq!(graph.edges, graph_from_str.edges);
    }

}