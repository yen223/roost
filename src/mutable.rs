use std::collections::{HashMap,HashSet};
use std::hash::Hash;

trait Graph<N>{
    fn contains_node(&self, node: N) -> bool;
}

trait MutableGraph<N>:Graph<N>{
    fn add_node(&mut self, node: N) -> bool;
    fn remove_node(&mut self, node:N) -> bool;
    fn add_edge(&mut self, from: N, to: N) -> bool;
}

struct SparseGraph<N: Hash+Eq>( HashMap<N, HashSet<N>> );
impl<N: Hash+Eq> SparseGraph<N>{
    fn new()->SparseGraph<N>{
        let map = HashMap::new();
        SparseGraph(map)
    }
}

impl<N: Hash+Eq> Graph<N> for SparseGraph<N> {
    fn contains_node(&self, node: N) -> bool{
        let &SparseGraph(ref map) = self;
        map.contains_key(&node)
    }
}

impl<N: Hash+Eq> MutableGraph<N> for SparseGraph<N> {
    fn add_node(&mut self, node: N) -> bool{
        let &SparseGraph(ref mut map) = self;
        if (!map.contains_key(&node)){
            let neighbours:HashSet<N> = HashSet::new();
            map.insert(node, neighbours)
        } else {
            false
        }
    }

    fn remove_node(&mut self, node:N) -> bool{
        let &SparseGraph(ref mut map) = self;
        map.remove(&node)
    }

    fn add_edge(&mut self, from:N, to:N) -> bool{
        self.add_node(from);
        self.add_node(to);
        let &SparseGraph(ref mut map) = self;
        let mut neighbours = map.find(&from);
            false

    }
}

#[test]
fn new_graph_with_int_nodes(){
    let mut gp:SparseGraph<int> = SparseGraph::new();
    gp.add_node(1);
    gp.add_node(2);
    gp.add_node(3);
    assert!(gp.contains_node(1));
    assert!(gp.contains_node(2));
    assert!(gp.contains_node(3));
    assert!(!gp.contains_node(4));
}

#[test]
fn new_graph_with_str_nodes(){
    let mut gp:SparseGraph<&str> = SparseGraph::new();
    gp.add_node("node a");
    gp.add_node("node b");
    gp.add_node("node c");
    assert!(gp.contains_node("node a"));
    assert!(gp.contains_node("node b"));
    assert!(gp.contains_node("node c"));
    assert!(!gp.contains_node("node z"));
}

#[test]
fn graph_remove_nodes(){
    let mut gp:SparseGraph<&str> = SparseGraph::new();
    gp.add_node("node a");
    gp.add_node("node b");
    gp.remove_node("node b");
    assert!(gp.contains_node("node a"));
    assert!(!gp.contains_node("node b"));
}
