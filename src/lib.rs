use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};
use std::hash::Hash;

trait Graph<N>{
    fn contains_node(&self, node: N) -> bool;
    fn contains_edge(&self, from: N, to: N) -> bool;
}

#[deriving(Hash, Eq, PartialEq, Clone)]
struct Edge<N>{
    from: N,
    to: N,
    weight: i64,
}

struct SparseGraph<N: Hash+Eq+Clone>( HashMap<N, HashMap<N, Edge<N>>>);
impl<N: Hash+Eq+Clone> SparseGraph<N>{
    fn new()->SparseGraph<N>{
        let map = HashMap::new();
        SparseGraph(map)
    }

    /*fn add_node(&mut self, n: N){
        let &SparseGraph(ref mut map) = self;
        match map.entry(n) {
            Occupied(_) => {},
            Vacant(entry) => {
                let hs: HashMap<N, Edge<N>> = HashMap::new();
                entry.set(hs);
            }
        };
    }*/

    fn add_edge(&mut self, e: Edge<N>){
        let from = e.from.clone();
        let to = e.to.clone();
        let &SparseGraph(ref mut map) = self;
        match map.entry(from.clone()) {
            Occupied(entry) => {
                let hs = entry.into_mut();
                hs.insert(to.clone(), e.clone());
            },
            Vacant(entry) => {
                let mut hs = HashMap::new();
                hs.insert(to.clone(), e.clone());
                entry.set(hs);
            }
        };
        match map.entry(to) {
            Occupied(entry) => {
                let hs = entry.into_mut();
                hs.insert(from, e.clone());
            },
            Vacant(entry) => {
                let mut hs = HashMap::new();
                hs.insert(from, e.clone());
                entry.set(hs);
            }
        };
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
        let &SparseGraph(ref map) = self;
        map.contains_key(&node)
    }

    fn contains_edge(&self, from: N, to: N) -> bool{
        let &SparseGraph(ref map) = self;
        match map.find(&from){
            Some(entry) => entry.contains_key(&to),
            None => false
        }
    }
}

#[test]
fn new_graph_with_str_nodes(){
    let edges = vec![
        Edge{from: "A", to: "B", weight: 1},
        Edge{from: "A", to: "C", weight: 2},
        Edge{from: "B", to: "C", weight: 1},
        Edge{from: "C", to: "D", weight: 1},
    ];
    let gp:SparseGraph<&str> = edges.into_iter().collect();
    assert!(gp.contains_node("A"));
    assert!(gp.contains_node("B"));
    assert!(gp.contains_node("C"));
    assert!(gp.contains_node("D"));
    assert!(!gp.contains_node("E"));

    assert!(gp.contains_edge("A", "B"));
    assert!(gp.contains_edge("B", "A")); //Simple undirected graph
    assert!(!gp.contains_edge("A", "D"));
}

#[test]
fn new_graph_with_int_nodes(){
    let edges = vec![
        Edge{from: 1i, to: 2, weight: 1},
        Edge{from: 1i, to: 3, weight: 2},
        Edge{from: 2i, to: 3, weight: 1},
        Edge{from: 3i, to: 4, weight: 1},
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
