extern crate roost;
use roost::graph::AdjList;
use roost::graph::Graph;

#[test]
fn graph_with_struct_nodes(){
    #[deriving(Eq, PartialEq, Clone)]
    struct Point(int, int);
    let point_a = Point( 1,3 );
    let point_b = Point( 4,-1 );
    let point_c = Point( 90,22 );
    let point_d = Point( 3,3 );
    let point_e = Point( 5,2 );
    let mut graph:AdjList<Point, int> = AdjList::new();
    graph.add_edge(point_a, point_b, 3);
    graph.add_edge(point_a, point_c, 1);
    graph.add_edge(point_b, point_c, 2);
    graph.add_edge(point_c, point_d, 6);
    assert!(graph.contains_node(&point_a));
    assert!(graph.contains_node(&point_b));
    assert!(graph.contains_node(&point_c));
    assert!(graph.contains_node(&point_d));
    assert!(!graph.contains_node(&point_e));
}
