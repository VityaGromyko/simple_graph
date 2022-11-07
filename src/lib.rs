extern crate core;

mod tests;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
pub struct Graph<VId, V, E> {
    vertices: HashMap<VId, V>,
    edges: HashMap<VId, Vec<(VId, E)>>,
}

impl<VId, V, E> Graph<VId, V, E>
where
    VId: Eq + Hash,
    V: Hash,
{
    /// create empty graph
    pub fn new() -> Graph<VId, V, E> {
        Graph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }
    /// add vertex (id, value)
    pub fn add_vertex(self: &mut Self, vid: VId, vertex: V) {
        self.vertices.insert(vid, vertex);
    }
    /// get vertices (id, value)
    pub fn get_vertices(self: &Self) -> &HashMap<VId, V> {
        &self.vertices
    }
    /// get value of vertice
    pub fn get_vertex_value(self: &Self, vid: VId) -> &V {
        &self.vertices.get(&vid).expect("Vertice not found")
    }
    /// remove vertex by id (and all edges)
    pub fn remove_vertex(self: &mut Graph<VId, V, E>, vid: VId) {
        self.vertices.remove(&vid);
        self.edges.remove(&vid);
        for (_, vector_with_edges) in self.edges.iter_mut() {
            let mut trash = Vec::new();
            for (index, (vid_to, _)) in vector_with_edges.iter().enumerate() {
                if *vid_to == vid {
                    trash.push(index)
                }
            }
            for index in trash.iter() {
                vector_with_edges.remove(*index);
            }
        }
    }
    /// add edge (from id, to id, edge value)
    pub fn add_edge(self: &mut Self, from: VId, to: VId, edge: E) {
        let adjacent_to_from = self.edges.entry(from).or_default();
        adjacent_to_from.push((to, edge));
    }
    /// get neighbors (vertixe id, edge value)
    pub fn get_neighbors(self: &Self, vid: VId) ->  Option<&Vec<(VId, E)>> {
            self.edges.get(&vid)
    }
    /// remove edge (id from, id to)
    pub fn remove_edge(self: &mut Self, from: VId, to: VId) {
        let edges = self
            .edges
            .get_mut(&from)
            .expect("edge not found in graph");
        let mut index = None;
        for (i, (vid_to, _)) in edges.iter().enumerate() {
            if *vid_to == to {
                index = Some(i)
            }
        }
        edges.remove(index.expect("edge not found in graph"));
    }
    /// Breadth First Search
    /// The time complexity of the BFS algorithm is represented in the form of O(V + E),
    /// where V is the number of nodes and E is the number of edges.
    pub fn bfs(self: &Self, start: VId, finish: VId) -> bool {
        let mut queue = Vec::new();
        let mut visited_vertexes = HashSet::new();
        queue.push(&start);
        visited_vertexes.insert(&start);
        while queue.len() > 0 {
            let vertex = queue.pop().unwrap();
            let neighbors = self.edges.get(vertex);
            if let Some(neighbors) = neighbors {
                for (vid, _) in neighbors {
                    if !visited_vertexes.contains(&vid) {
                        queue.push(&vid);
                        visited_vertexes.insert(&vid);
                        if *vid == finish {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

impl<VId, V, E> Graph<VId, V, E>
where
    VId: Eq + Hash + Debug,
    V: Hash + Debug,
    E: Debug,
{
    /// serialize Graph into TGF (Trivial Graph Format)
    pub fn serialize(self: &Self) -> String {
        let mut output = String::new();
        for (vid, vertex) in self.vertices.iter() {
            output += &format!("{:?} {:?}\n", &vid, &vertex);
        }
        output += "#\n";
        for (from, edges) in self.edges.iter() {
            for (to, edge) in edges.iter() {
                output += &format!("{:?} {:?} {:?}\n", from, to, edge);
            }
        }
        output
    }
}

impl<VId, V, E> Graph<VId, V, E>
where
    VId: Eq + Hash + Debug + FromStr,
    V: Hash + Debug + FromStr,
    E: Debug + FromStr,
{
    /// deserialize Graph from TGF (Trivial Graph Format)
    pub fn from(data: &str) -> Self
    where
        <VId as FromStr>::Err: Debug,
        <V as FromStr>::Err: Debug,
        <E as FromStr>::Err: Debug,
    {
        /// delete quotes (") from start and end of string
        /// example:
        /// "hello world" => hello world
        /// "test => "test or test" => test"
        fn delete_quotes(line: &str) -> &str {
            if line.starts_with("\"") && line.ends_with("\"") {
                return &line[1..(line.len() - 1)];
            }
            line
        }
        /// split line into Vec with custom length by whitespaces
        /// example:
        /// line="praise the blobcat" n=2 => ["praise", "the blobcat"]
        /// line="praise the blobcat" n=3 => ["praise", "the", "blobcat"]
        fn split_line_into_vec_with_len(line: &str, n: usize) -> Vec<String> {
            let data: Vec<String> = line.split_ascii_whitespace().map(|x|x.into()).collect();
            if data.len() < n {
                panic!("Error wrong data")
            } else if data.len() == n {
                return data;
            } else {
                let mut tmp: Vec<String> = Vec::new();
                for i in 0..n-1 {
                    tmp.push(data[i].clone());
                }
                tmp.push(data[n-1..].join(" "));
                return tmp
            }
        }
        let mut output : Graph<VId, V, E>= Graph::new();
        let data: Vec<&str> = data.lines().collect();
        let separator = data.iter().position(|x| x == &"#").expect("data corrupted");

        for (i, line) in data.iter().enumerate() {
            match i {
                i if i < separator => {
                    // line elements = [Vid, V]
                    let line_elements = split_line_into_vec_with_len(line, 2);
                    let vid = delete_quotes(&line_elements[0]).parse::<VId>().unwrap();
                    let vertex = delete_quotes(&line_elements[1]).parse::<V>().unwrap();
                    output.add_vertex(vid, vertex);
                }
                i if i > separator => {
                    // line elements = [from, to, edge]
                    let line_elements = split_line_into_vec_with_len(line, 3);
                    let from = delete_quotes(&line_elements[0]).parse::<VId>().unwrap();
                    let to = delete_quotes(&line_elements[1]).parse::<VId>().unwrap();
                    let edge = delete_quotes(&line_elements[2]).parse::<E>().unwrap();
                    output.add_edge(from, to, edge);
                },
                _ => (),
            }
        }
        output
    }
    pub fn from_file(filepath: &str) -> Self
        where
        <VId as FromStr>::Err: Debug,
        <V as FromStr>::Err: Debug,
        <E as FromStr>::Err: Debug,
    {
        let mut file= File::open(filepath).expect("Can't open file");
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).expect("Can't read file data");
        Graph::from(&file_data)
    }
}
