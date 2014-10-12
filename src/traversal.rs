use roost::{Graph, SparseGraph, NodeIndex, EdgeIndex};
use std::collections::{HashSet, DList, Deque};

struct BreadthFirstVisit<'a, V, E, G>
    where V: Clone+Eq,
          G: 'a + Graph<V,E>,
{
    gp: &'a G,
    next_edges: DList<EdgeIndex>,
    visited: HashSet<NodeIndex>,
}

struct DepthFirstVisit<'a, V, E, G>
    where V: Clone+Eq,
          G: 'a + Graph<V,E>,
{
        gp: &'a G,
        next_edges: Vec<EdgeIndex>,
        visited: HashSet<NodeIndex>,
}

pub trait Searchable<V,E>:Graph<V,E> 
    where V: Clone+Eq,
{
    fn breadth_first_visit(&self, root: &V)->BreadthFirstVisit<V, E, Self>{
        let mut visit:HashSet<NodeIndex> = HashSet::new();
        let mut queue = DList::new();
        match self.index_of(root){
            Some(x) => {
                visit.insert(x);
                for &nb in self.out_nodes(x).iter(){
                    queue.push((x, nb));
                }
            },
            None    => {}
        }
        BreadthFirstVisit{gp: self, next_edges: queue, visited: visit}
    }

    fn depth_first_visit(&self, root: &V)-> DepthFirstVisit<V, E, Self>{
        let mut visit:HashSet<NodeIndex> = HashSet::new();
        let mut stack = Vec::new(); 
        match self.index_of(root){
            Some(x) => {
                visit.insert(x);
                for &nb in self.out_nodes(x).iter().rev(){
                    stack.push((x, nb));
                }
            },
            None    => {}
        }
        DepthFirstVisit{gp: self, next_edges: stack, visited: visit}
    }
}

impl<'a, N, E, G> Iterator<EdgeIndex> for BreadthFirstVisit<'a, N, E, G>
    where N: Clone+Eq,
          G: Graph<N, E>,
{
    fn next(&mut self) -> Option<EdgeIndex>{
        let curr = self.next_edges.pop_front();
        match curr {
            Some((_, node)) => {
                let out_nodes = self.gp.out_nodes(node);
                for n in out_nodes.into_iter(){
                    if !self.visited.contains(&n){
                        self.visited.insert(n);
                        self.next_edges.push((node, n));
                    }
                }
            },
            None => {}
        }
        curr
    }
}

impl<'a, N, E, G> Iterator<EdgeIndex> for DepthFirstVisit<'a, N, E, G>
    where N: Clone+Eq,
          G: Graph<N, E>
{
    fn next(&mut self) -> Option<EdgeIndex>{
        let mut curr;
        loop {
            curr = self.next_edges.pop();
            match curr {
                Some((_, node)) if !self.visited.contains(&node) => {
                    self.visited.insert(node);
                    let out_nodes = self.gp.out_nodes(node);
                    for n in out_nodes.into_iter().rev(){
                        self.next_edges.push((node, n));
                    }
                    break
                },
                Some(_) => {}
                None    => {break}
            }
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
    graph.add_edge(1i, 4i, 1);
    graph.add_edge(4i, 5i, 4);
    graph.add_edge(4i, 6i, 6);
    graph.add_edge(5i, 7i, 7);
    let bfs:Vec<(int, int)> = graph.breadth_first_visit(&1i)
                            .map(|(x,y)|{
                                (graph.node_of(x).unwrap(), graph.node_of(y).unwrap())
                            })
                            .collect();
    assert_eq!(bfs, vec![(1i, 2i), 
                         (1i, 3i), 
                         (1i, 4i),
                         (4i, 5i),
                         (4i, 6i),
                         (5i, 7i)]);
}

#[test]
fn depth_first_search(){
    let mut graph:SparseGraph<int, int> = SparseGraph::new();
    graph.add_edge(1i, 2i, 1);
    graph.add_edge(1i, 3i, 2);
    graph.add_edge(2i, 3i, 4);
    graph.add_edge(1i, 4i, 1);
    graph.add_edge(3i, 5i, 4);
    graph.add_edge(3i, 6i, 6);
    graph.add_edge(5i, 7i, 7);
    let dfs:Vec<(int, int)> = graph.depth_first_visit(&1i)
                            .map(|(x, y)|{
                                (graph.node_of(x).unwrap(), graph.node_of(y).unwrap())
                            })
                            .collect();
    assert_eq!(dfs, vec![(1i, 2i), 
                         (2i, 3i),
                         (3i, 5i),
                         (5i, 7i),
                         (3i, 6i),
                         (1i, 4i),]);
}

