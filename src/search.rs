use roost::{Graph, SparseGraph, NodeIndex, EdgeIndex};
use std::collections::{HashSet, RingBuf, Deque};
use std::hash::Hash;
use std::iter::Iterator;


struct BreadthFirstVisit<'a, V:Clone+Eq, E, G:'a + Graph<V, E>>{
    gp: &'a G,
    next_nodes: RingBuf<NodeIndex>,
    visited: HashSet<NodeIndex>,
}

struct DepthFirstVisit<'a, V:Clone+Eq, E, G:'a + Graph<V, E>>{
    gp: &'a G,
    next_nodes: Vec<NodeIndex>,
    visited: HashSet<NodeIndex>,
}

pub trait Searchable<V:Clone+Eq,E>:Graph<V,E> {
    fn breadth_first_visit(&self, root: &V)->BreadthFirstVisit<V, E, Self>{
        let mut visit:HashSet<NodeIndex> = HashSet::new();
        let mut queue = RingBuf::new();
        match self.index_of(root){
            Some(x) => {
                visit.insert(x);
                queue.push(x);
            },
            None    => {}
        }
        BreadthFirstVisit{gp: self, next_nodes: queue, visited: visit}
    }

    fn depth_first_visit(&self, root: &V)-> DepthFirstVisit<V, E, Self>{
        let mut visit:HashSet<NodeIndex> = HashSet::new();
        let mut stack = Vec::new(); 
        match self.index_of(root){
            Some(x) => {
                visit.insert(x);
                stack.push(x);
            },
            None    => {}
        }
        DepthFirstVisit{gp: self, next_nodes: stack, visited: visit}
    }
}


impl<'a, N:Clone+Eq, E, G: Graph<N, E>> Iterator<NodeIndex> for BreadthFirstVisit<'a, N, E, G>{
    fn next(&mut self) -> Option<NodeIndex>{
        let curr = self.next_nodes.pop_front();
        match curr {
            Some(node) => {
                let neighbors = self.gp.neighbors(node);
                for n in neighbors.into_iter(){
                    if !self.visited.contains(&n){
                        self.visited.insert(n);
                        self.next_nodes.push(n);
                    }
                }
            },
            None => {}
        }
        curr
    }
}

impl<'a, N:Clone+Eq, E, G: Graph<N, E>> Iterator<NodeIndex> for DepthFirstVisit<'a, N, E, G>{
    fn next(&mut self) -> Option<NodeIndex>{
        let curr = self.next_nodes.pop();
        match curr {
            Some(node) => {
                let neighbors = self.gp.neighbors(node);
                for n in neighbors.into_iter(){
                    if !self.visited.contains(&n){
                        self.visited.insert(n);
                        self.next_nodes.push(n);
                    }
                }
            },
            None => {}
        }
        curr
    }
}


impl <V:Clone+Eq, E:Clone> Searchable<V,E> for SparseGraph<V,E>{}

#[test]
fn breadth_first_search(){
    let mut graph:SparseGraph<int, int> = SparseGraph::new();
    graph.add_edge(1i, 2i, 1);
    graph.add_edge(1i, 3i, 2);
    graph.add_edge(1i, 6i, 1);
    graph.add_edge(2i, 5i, 4);
    graph.add_edge(3i, 6i, 6);
    graph.add_edge(3i, 7i, 7);
    let bfs:Vec<int> = graph.breadth_first_visit(&1i)
                            .map(|x|{graph.node_of(x).unwrap()})
                            .collect();
    assert_eq!(bfs, vec![1, 2, 3, 6, 5, 7]);
}

#[test]
fn depth_first_search(){
    let mut graph:SparseGraph<int, int> = SparseGraph::new();
    graph.add_edge(1i, 2i, 1);
    graph.add_edge(1i, 3i, 2);
    graph.add_edge(1i, 4i, 1);
    graph.add_edge(4i, 5i, 4);
    graph.add_edge(4i, 6i, 6);
    graph.add_edge(5i, 7i, 7);
    let dfs:Vec<int> = graph.depth_first_visit(&1i)
                            .map(|x|{graph.node_of(x).unwrap()})
                            .collect();
    assert_eq!(dfs, vec![1, 4, 6, 5, 7, 3, 2]);
}
