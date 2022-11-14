use simple_graph::*;

fn main() {
    let graph: Graph<String, i32, f64> = Graph::from_file("graph.tgf");
    println!("{:?}\n", graph);

    for (id, value) in graph.get_vertices() {
        println!("vertex id: {} | vertex value: {}", id, value);
        if let Some(neighbors) = graph.get_neighbors(id.to_string()) {
            for (neighbor_id, edge_value) in neighbors {
                let neighbor_value = graph.get_vertex_value(neighbor_id.to_string()).unwrap();
                println!(
                    "    neighbor_id: {} | neighbor_value: {} | edge_value: {}",
                    neighbor_id, neighbor_value, edge_value
                );
            }
        } else {
            println!("    neighbors not found");
        }
    }

    println!(
        "\npath between A and C: {}",
        graph.bfs("A".to_string(), "C".to_string())
    );
}
