use roost::{Graph, SparseGraph, NodeIndex, EdgeIndex};
use roost::edge::DistanceEdge;
use std::collections::{HashSet, RingBuf, DList, Deque, PriorityQueue, };
use std::iter::{Iterator, DoubleEndedIterator};


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

#[deriving(Clone)]
struct NodeDist(NodeIndex, f64);

impl Ord for NodeDist{
    fn cmp(&self, other:&NodeDist)->Ordering{
        // The compare function is inverted,
        // so that the max-heap becomes a
        // min-heap.
        let NodeDist(_, dist_a) = *self;
        let NodeDist(_, dist_b) = *other;
        match dist_b.partial_cmp(&dist_a){
            Some(o) => o,
            None    => Equal,
        }
    }
}

impl PartialOrd for NodeDist{
    fn partial_cmp(&self, other:&NodeDist) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for NodeDist{
    fn eq(&self, other:&NodeDist) -> bool{
        let NodeDist(_, dist_a) = *self;
        let NodeDist(_, dist_b) = *other;
        dist_a.eq(&dist_b)
    }
}
impl Eq for NodeDist{}

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

pub trait PathSearchable<N, V, E>: Graph<V, E> 
    where N: Num+ToPrimitive,
          V: Clone+Eq,
          E: DistanceEdge<N>,
{
    fn dijkstra_shortest_path(&self, source: NodeIndex, target: NodeIndex) -> Option<Vec<NodeIndex>>{
        let node_len = self.nodes().len();
        let mut dist:Vec<f64> = Vec::from_elem(node_len, Float::infinity());
        let mut prev:Vec<Option<NodeIndex>> = Vec::from_elem(node_len, None);
        let mut queue:PriorityQueue<NodeDist> = PriorityQueue::new();
        queue.push(NodeDist(source, 0.0));
        *dist.get_mut(source) = 0.0;
        loop {
            match queue.pop(){
                Some(NodeDist(u, dist_u)) => {
                    for &v in self.out_nodes(u).iter(){
                        let dist_edge:f64 = self.get_edge(u, v)
                                                .unwrap()
                                                .distance()
                                                .to_f64()
                                                .unwrap();
                        let alt:f64 = dist_u + dist_edge;
                        if alt < dist[v]{
                            *dist.get_mut(v) = dist_u + dist_edge;
                            *prev.get_mut(v) = Some(u);
                        }
                        queue.push(NodeDist(v, dist[v])); 
                    }
                },
                None    => break,
            }
        }
        let mut path:DList<NodeIndex> = DList::new();
        path.push_front(target);
        let mut curr = target;
        loop {
            match prev[curr]{
                Some(parent) if parent == source => {
                    path.push_front(parent);
                    break;
                },
                Some(parent) => {
                    path.push_front(parent);
                    curr = parent;
                },
                None         => {
                    return None
                },
            }
        }
        let result:Vec<NodeIndex> = path.into_iter().collect();
        Some(result)

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
        let curr = self.next_edges.pop();
        match curr {
            Some((_, node)) if !self.visited.contains(&node) => {
                self.visited.insert(node);
                let out_nodes = self.gp.out_nodes(node);
                for n in out_nodes.into_iter().rev(){
                    self.next_edges.push((node, n));
                }
            },
            Some(_) => {}
            None    => {}
        }
        curr
    }
}


impl <V:Clone+Eq, E:Clone> Searchable<V,E> for SparseGraph<V,E>{}
impl <N:Num+ToPrimitive, V: Clone+Eq, E:DistanceEdge<N>> PathSearchable<N, V, E> for SparseGraph<V, E>{}


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

#[test]
fn dist_pair_comparisons(){
    let a = NodeDist(10u, 6.6);
    let b = NodeDist(24u, 0.0);
    let c = NodeDist(15u, 6.6);
    let d = NodeDist(29u, Float::infinity());
    assert!(b > a);
    assert!(c == a);
    assert!(d < a);
    assert!(b > d);
}

#[test]
fn dijkstra_with_path(){
    #[deriving(Eq, PartialEq, Clone)]
    struct DistEdge{
        d: uint,
    }

    impl DistanceEdge<uint> for DistEdge{
        fn distance(&self) -> uint{
            self.d
        }
    }

    let mut graph:SparseGraph<&str, DistEdge> = SparseGraph::new();
    graph.add_edge("a", "b", DistEdge{d: 7});
    graph.add_edge("a", "c", DistEdge{d: 9});
    graph.add_edge("a", "f", DistEdge{d: 14});
    graph.add_edge("b", "c", DistEdge{d: 10});
    graph.add_edge("b", "d", DistEdge{d: 15});
    graph.add_edge("c", "d", DistEdge{d: 11});
    graph.add_edge("c", "f", DistEdge{d: 2});
    graph.add_edge("d", "e", DistEdge{d: 6});
    graph.add_edge("e", "f", DistEdge{d: 9});

    let src_idx = graph.index_of(&"a").unwrap();
    let trg_idx = graph.index_of(&"e").unwrap();
    let path:Vec<&str> = graph.dijkstra_shortest_path(src_idx, trg_idx).unwrap()
                            .iter()
                            .map(|&x|{graph.node_of(x).unwrap()})
                            .collect();
    assert_eq!(path, vec!["a", "c", "d", "e"])
}
