use graph::{Graph, AdjList, NodeIndex, Node};
use graph::error::{GraphError, UnknownError};
use graph::edge::DistanceEdge;
use graph::traversal::Traverseable;
use std::collections::{DList, BinaryHeap };
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
        match target {
            Ok(target_idx) => self.breadth_first_visit(source)
                                  .any(|(_, to)|{to == target_idx}),
            Err(_)         => false,
        }
    }

    fn dijkstra_shortest_path(&self, source: Node, target: Node) -> Result<Vec<Node>, GraphError>{
        let src_idx = try!(source);
        let trg_idx = try!(target);
        let node_len = self.nodes().len();
        let mut dist:Vec<f64> = Vec::from_elem(node_len, Float::infinity());
        let mut prev:Vec<Option<NodeIndex>> = Vec::from_elem(node_len, None);
        let mut queue:BinaryHeap<NodeDist> = BinaryHeap::new();
        queue.push(NodeDist(src_idx, 0.0));
        dist[src_idx] = 0.0;
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
                            dist[v] = dist_u + dist_edge;
                            prev[v] = Some(u);
                        }
                        queue.push(NodeDist(v, dist[v])); 
                    }
                },
                None    => break,
            }
        }
        let mut path:DList<NodeIndex> = DList::new();
        path.push_front(trg_idx);
        let mut curr = trg_idx;
        loop {
            match prev[curr]{
                Some(parent) if parent == src_idx => {
                    path.push_front(parent);
                    break;
                },
                Some(parent) => {
                    path.push_front(parent);
                    curr = parent;
                },
                None         => {
                    return Err(UnknownError);
                },
            }
        }
        let result:Vec<Node> = path.into_iter().map(|x|{Ok(x)}).collect();
        Ok(result)
    }
}

impl <N:Num+ToPrimitive, V: Clone+Eq, E:DistanceEdge<N>> Path<N, V, E> for AdjList<V, E>{}

 #[cfg(test)]
mod test{
    use graph::{Graph, AdjList, node};
    use graph::edge::{UnitEdge, DistanceEdge};
    use graph::path::{NodeDist, Path};
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
        let mut graph:AdjList<&str, UnitEdge> = AdjList::new();
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

        assert!(graph.has_path(node(&"a", &graph), 
                               node(&"d", &graph)));
        assert!(!graph.has_path(node(&"a", &graph),
                                node(&"z", &graph)));
                                
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

        let mut gp:AdjList<&str, DistEdge> = AdjList::new();
        gp.add_edge("a", "b", DistEdge{d: 7});
        gp.add_edge("a", "c", DistEdge{d: 9});
        gp.add_edge("a", "f", DistEdge{d: 14});
        gp.add_edge("b", "c", DistEdge{d: 10});
        gp.add_edge("b", "d", DistEdge{d: 15});
        gp.add_edge("c", "d", DistEdge{d: 11});
        gp.add_edge("c", "f", DistEdge{d: 2});
        gp.add_edge("d", "e", DistEdge{d: 6});
        gp.add_edge("e", "f", DistEdge{d: 9});

        // let src_idx = gp.node_to_index(&"a").ok().unwrap();
        // let trg_idx = gp.node_to_index(&"e").ok().unwrap();
        let path:Vec<&str> = gp.dijkstra_shortest_path(
                                node(&"a", &gp),
                                node(&"e", &gp)
                                ).unwrap()
                                .iter()
                                .map(|&x|{gp.index_to_node(x.unwrap()).unwrap()})
                                .collect();
        assert_eq!(path, vec!["a", "c", "d", "e"])
    }
}
