use roost::{Graph, SparseGraph, NodeIndex, Node};
use roost::edge::DistanceEdge;
use roost::traversal::Traverseable;
use std::collections::{DList, Deque, PriorityQueue, };
use std::iter::{Iterator};

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

pub trait Path<N, V, E>: Traverseable<V, E> 
    where N: Num+ToPrimitive,
          V: Clone+Eq,
          E: DistanceEdge<N>,
{
    fn has_path(&self, source: Node, target: Node) -> bool {
        let target_idx = target.expect("Target not found in graph.");
        self.breadth_first_visit(source).any(|(_, to)|{to == target_idx})
    }

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

impl <N:Num+ToPrimitive, V: Clone+Eq, E:DistanceEdge<N>> Path<N, V, E> for SparseGraph<V, E>{}

 #[cfg(test)]
mod test{
    use roost::{Graph, SparseGraph, node};
    use roost::edge::{UnitEdge, DistanceEdge};
    use roost::path::{NodeDist, Path};
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
    fn graph_has_path(){
        let mut graph:SparseGraph<&str, UnitEdge> = SparseGraph::new();
        graph.add_edge("a", "b", UnitEdge);
        graph.add_edge("a", "c", UnitEdge);
        graph.add_edge("a", "f", UnitEdge);
        graph.add_edge("b", "c", UnitEdge);
        graph.add_edge("b", "d", UnitEdge);
        graph.add_edge("c", "d", UnitEdge);
        graph.add_edge("c", "f", UnitEdge);
        graph.add_edge("d", "e", UnitEdge);
        graph.add_edge("e", "f", UnitEdge);
        graph.add_edge("y", "z", UnitEdge);

        assert!(graph.has_path(node(&graph, &"a"), 
                               node(&graph, &"d")));
        assert!(!graph.has_path(node(&graph, &"a"),
                                node(&graph, &"z")));
                                
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
}
