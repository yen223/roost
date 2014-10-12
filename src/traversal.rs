use roost::{Graph, SparseGraph, Node, NodeIndex, EdgeIndex, node};
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

pub trait Traverseable<V,E>:Graph<V,E> 
    where V: Clone+Eq,
{
    fn breadth_first_visit(&self, root: Node) -> BreadthFirstVisit<V, E, Self>{
        let mut visit:HashSet<NodeIndex> = HashSet::new();
        let mut queue = DList::new();
        match root{
            Ok(root_idx) => {
                visit.insert(root_idx);
                for &nb in self.out_nodes(root_idx).iter(){
                    queue.push((root_idx, nb));
                }
            },
            Err(_)       => {}
        };
        BreadthFirstVisit{gp: self, next_edges: queue, visited: visit}
    }

    fn depth_first_visit(&self, root: Node)-> DepthFirstVisit<V, E, Self>{
        let root_idx = root.ok().expect("Root node does not exist in graph.");
        let mut visit:HashSet<NodeIndex> = HashSet::new();
        let mut stack = Vec::new(); 
        visit.insert(root_idx);
        for &nb in self.out_nodes(root_idx).iter().rev(){
            stack.push((root_idx, nb));
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

impl <V:Clone+Eq, E:Clone> Traverseable<V,E> for SparseGraph<V,E>{}

#[cfg(test)]
mod test {
    use roost::{Graph, SparseGraph, Node, EdgeIndex, node};
    use roost::traversal::Traverseable;
    #[test]
    fn breadth_first_search(){
        let mut gp:SparseGraph<int, int> = SparseGraph::new();
        gp.add_edge(1i, 2i, 1);
        gp.add_edge(1i, 3i, 2);
        gp.add_edge(1i, 4i, 1);
        gp.add_edge(4i, 5i, 4);
        gp.add_edge(4i, 6i, 6);
        gp.add_edge(5i, 7i, 7);
        let bfs:Vec<(int, int)> = gp.breadth_first_visit(node(&1i, &gp))
                                    .map(|(x,y)|{
                                            (gp.index_to_node(x).unwrap(), 
                                             gp.index_to_node(y).unwrap())
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
        let mut gp:SparseGraph<int, int> = SparseGraph::new();
        gp.add_edge(1i, 2i, 1);
        gp.add_edge(1i, 3i, 2);
        gp.add_edge(2i, 3i, 4);
        gp.add_edge(1i, 4i, 1);
        gp.add_edge(3i, 5i, 4);
        gp.add_edge(3i, 6i, 6);
        gp.add_edge(5i, 7i, 7);
        let dfs:Vec<(int, int)> = gp.depth_first_visit(node(&1i, &gp))
                                .map(|(x, y)|{
                                    (gp.index_to_node(x).unwrap(), gp.index_to_node(y).unwrap())
                                })
                                .collect();
        assert_eq!(dfs, vec![(1i, 2i), 
                             (2i, 3i),
                             (3i, 5i),
                             (5i, 7i),
                             (3i, 6i),
                             (1i, 4i),]);
    }
}
