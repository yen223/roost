use std::collections::{HashMap, HashSet};
use std::collections::hashmap::{Occupied, Vacant};
use std::hash::Hash;
use std::vec::Vec;
use roost::{Graph, Edge};

pub struct SparseGraph<N: Clone>{
    nodes: Vec<N>,
    edges: HashMap<(uint,uint),Edge<N>>,
    adj_list: Vec<Vec<uint>>,
}

impl<N: Eq+Clone> SparseGraph<N>{
    fn new()->SparseGraph<N>{
        let n:Vec<N> = Vec::new();
        let e:HashMap<(uint, uint), Edge<N>> = HashMap::new();
        let adjl:Vec<Vec<uint>> = Vec::new();
        SparseGraph{nodes: n, edges: e, adj_list: adjl}
    }

    fn add_node(&mut self, n: N){
        self.nodes.push(n);
        self.adj_list.push(Vec::new());
    }

    fn get_node_index(&self, n: &N) -> Option<uint> {
        let nds = self.nodes.as_slice();
        for (idx, x) in nds.iter().enumerate(){
            if *x == *n {return Some(idx);}
        }
        return None;
    }

    fn add_edge(&mut self, edge: Edge<N>){
        let fi = self.get_node_index(&(edge.from));
        let from:uint = match fi{
            Some(x) => x,
            None    => {
                self.add_node(edge.from.clone());
                self.nodes.len()-1
            },
        };
        let ti = self.get_node_index(&(edge.to));
        let to:uint = match ti{
            Some(x) => x,
            None    => {
                self.add_node(edge.to.clone());
                self.nodes.len()-1
            }
        };
        self.edges.insert((from, to), edge);
        self.adj_list.get_mut(from).push(to);
    }
}

impl<N: Hash+Eq+Clone> FromIterator<Edge<N>> for SparseGraph<N>{
    fn from_iter<T: Iterator<Edge<N>>>(mut iterator: T) -> SparseGraph<N>{
        let mut graph:SparseGraph<N> = SparseGraph::new();
        for edge in iterator{
            graph.add_edge(edge);
        }
        graph
    }
}

impl<N: Hash+Eq+Clone> Graph<N> for SparseGraph<N> {
    fn contains_node(&self, node: N) -> bool{
        match self.get_node_index(&node){
            Some(_) => true,
            None    => false,
        }
    }

    fn contains_edge(&self, from: N, to: N) -> bool{
        let fi = match self.get_node_index(&from){
            Some(x)     => x,
            None        => return false,
        };

        let ti = match self.get_node_index(&to){
            Some(x)     => x,
            None        => return false,
        };
        !self.edges.find(&(fi, ti)).is_none()
    }

    fn neighbors(&self, node:N)->Vec<N>{
        let idx = self.get_node_index(&node);
        let neighbors:Vec<N> = match idx{
            Some(entry) => self.adj_list.get(entry)
                                   .iter()
                                   .map(|&x|self.nodes.get(x).clone())
                                   .collect(),
            None        => Vec::new(),
        };
        neighbors
    }
}

#[test]
fn new_graph_with_str_nodes(){
    let edges = vec![
        Edge{from: "A", to: "B", weight: 1.1},
        Edge{from: "A", to: "C", weight: 2.3},
        Edge{from: "B", to: "C", weight: 1.2},
        Edge{from: "C", to: "D", weight: 1.5},
    ];
    let gp:SparseGraph<&str> = edges.into_iter().collect();
    assert!(gp.contains_node("A"));
    assert!(gp.contains_node("B"));
    assert!(gp.contains_node("C"));
    assert!(gp.contains_node("D"));
    assert!(!gp.contains_node("E"));

    assert!(gp.contains_edge("A", "B"));
    assert!(!gp.contains_edge("A", "D"));
    let a_neighbors = gp.neighbors("A");
    println!("Neighbors: {}", a_neighbors);
    assert!(a_neighbors.contains(&"B"));
    assert!(a_neighbors.contains(&"C"));
    assert!(!a_neighbors.contains(&"D"));

    let mut gp_mut = gp;
    assert!(!gp_mut.contains_node("E"));
    gp_mut.add_node("E");
    assert!(gp_mut.contains_node("E"));
}

#[test]
fn new_graph_with_int_nodes(){
    let edges = vec![
        Edge{from: 1i, to: 2, weight: 1.0},
        Edge{from: 1i, to: 3, weight: 2.5},
        Edge{from: 2i, to: 3, weight: 1.0},
        Edge{from: 3i, to: 4, weight: 1.0},
    ];
    let gp:SparseGraph<int> = edges.into_iter().collect();
    assert!(gp.contains_node(1));
    assert!(gp.contains_node(2));
    assert!(gp.contains_node(3));
    assert!(gp.contains_node(4));
    assert!(!gp.contains_node(5));

    assert!(gp.contains_edge(1i, 2));
    assert!(!gp.contains_edge(1, 4));
}
