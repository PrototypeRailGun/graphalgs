//! Algorithms that simplify work with such type of graphs as a 
//! [tournament](https://en.wikipedia.org/wiki/Tournament_(graph_theory)).

use std::hash::Hash;
use std::collections::HashSet;
use petgraph::visit::{EdgeRef, IntoEdgeReferences, NodeCount };
use crate::traits::EdgeCount;


/// Is the graph a [tournament](https://en.wikipedia.org/wiki/Tournament_(graph_theory))?
/// 
/// This function checks if the given graph is a tournament and returns the corresponding boolean value.
/// 
/// # Examples
/// 
/// ```
/// use graphalgs::tournament::is_tournament;
/// use petgraph::Graph;
/// 
/// let graph = Graph::<(), ()>::from_edges(&[
///     (0, 2), (1, 0), (1, 2), 
///     (3, 0), (3, 1), (3, 2),
/// ]);
/// 
/// assert!(is_tournament(&graph));
/// 
/// let graph = Graph::<(), ()>::from_edges(&[
///     (0, 2), (1, 0), (1, 2), 
///     (3, 0), (3, 1), (1, 3),
/// ]);
/// 
/// // The graph is not a tournament, because it contains a pair of opposite edges.
/// assert!(!is_tournament(&graph));
/// ```
pub fn is_tournament<G>(graph: G) -> bool
where 
    G: IntoEdgeReferences + NodeCount + EdgeCount,
    G::NodeId: Eq + Hash,
{
    // Tournament has |V|*(|V|-1)/2 edges.
    if 2*graph.number_of_edges() != graph.node_count() * (graph.node_count()-1) {
        return false;   
    }

    let mut edges: HashSet<(G::NodeId, G::NodeId)> = HashSet::new();

    for edge in graph.edge_references() {
        let source = edge.source();
        let target = edge.target();

        if source == target || 
            edges.contains(&(target, source)) ||
            edges.contains(&(source, target))  {
            return false;
        }

        edges.insert((source, target));
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Directed;
    use petgraph::Graph;
    use petgraph::stable_graph::StableGraph;
    use petgraph::graphmap::GraphMap;
    use petgraph::matrix_graph::MatrixGraph;
    use petgraph::csr::Csr;

    fn graph1() -> Graph<(), ()> {
        let mut graph = Graph::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        graph.add_node(());
        graph.add_edge(n0, n1, ());
        graph
    }

    fn graph2() -> Graph<(), ()> {
        let mut graph = Graph::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        graph.add_edge(n0, n1, ());
        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n0, ());
        graph.add_edge(n2, n1, ());
        graph
    }
    fn graph3() -> Graph<(), ()> {
        Graph::<(), ()>::from_edges(&[
            (0, 2), (1, 0), (1, 2), 
            (3, 0), (3, 1), (1, 2),
        ])
    }

    fn graph4() -> Graph<(), ()> {
        Graph::<(), ()>::from_edges(&[
            (0, 2), (1, 0), (1, 2), 
            (3, 0), (3, 1), (1, 1),
        ])
    }

    fn graph_tour() -> Graph<(), i32> {
        let mut graph = Graph::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        graph.add_edge(n0, n1, 0);
        graph.add_edge(n1, n2, 1);
        graph.add_edge(n2, n0, 2);
        graph
    }

    fn stable_graph_tour() -> StableGraph<(), i32> {
        let mut graph = StableGraph::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        graph.add_edge(n0, n1, 0);
        graph.add_edge(n1, n2, 1);
        graph.add_edge(n2, n0, 2);
        graph
    }

    fn graphmap_tour() -> GraphMap<i8, i32, Directed> {
        let mut graph = GraphMap::new();
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        graph.add_edge(n0, n1, 1);
        graph.add_edge(n1, n2, 3);
        graph.add_edge(n2, n0, 2);
        graph
    }

    fn matrix_graph_tour() -> MatrixGraph<(), i32> {
        let mut graph = MatrixGraph::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        graph.add_edge(n0, n1, 0);
        graph.add_edge(n1, n2, 1);
        graph.add_edge(n2, n0, 2);
        graph
    }

    fn csr_tour() -> Csr<(), i32> {
        let mut graph = Csr::new();
        let n0 = graph.add_node(());
        let n1 = graph.add_node(());
        let n2 = graph.add_node(());
        graph.add_edge(n0, n1, 0);
        graph.add_edge(n1, n2, 1);
        graph.add_edge(n2, n0, 2);
        graph
    }
    
    #[test]
    fn test_is_tournament() {
        // At the same time testing EdgeCount trait.
        assert!(!is_tournament(&graph1()));
        assert!(!is_tournament(&graph2()));
        assert!(!is_tournament(&graph3()));
        assert!(!is_tournament(&graph4()));
        assert!(is_tournament(&graph_tour()));
        assert!(is_tournament(&stable_graph_tour()));
        assert!(is_tournament(&graphmap_tour()));
        assert!(is_tournament(&matrix_graph_tour()));
        assert!(is_tournament(&csr_tour()));
    }
}