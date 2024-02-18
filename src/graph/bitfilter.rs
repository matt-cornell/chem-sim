use petgraph::data::*;
use petgraph::visit::*;
use petgraph::Direction;
use smallbitvec::SmallBitVec;

#[derive(Debug, Clone)]
pub struct BitFiltered<G> {
    pub graph: G,
    pub filter: SmallBitVec,
}
impl<G> BitFiltered<G> {
    pub const fn new(graph: G, filter: SmallBitVec) -> Self {
        Self { graph, filter }
    }
}

impl<G: GraphBase> GraphBase for BitFiltered<G> {
    type EdgeId = G::EdgeId;
    type NodeId = G::NodeId;
}

impl<G: GraphProp> GraphProp for BitFiltered<G> {
    type EdgeType = G::EdgeType;
    fn is_directed(&self) -> bool {
        self.graph.is_directed()
    }
}

impl<G: Data> Data for BitFiltered<G> {
    type NodeWeight = G::NodeWeight;
    type EdgeWeight = G::EdgeWeight;
}

impl<G: DataMap + NodeIndexable> DataMap for BitFiltered<G> {
    fn node_weight(&self, id: Self::NodeId) -> Option<&Self::NodeWeight> {
        self.filter
            .get(self.graph.to_index(id))?
            .then(|| self.graph.node_weight(id))?
    }
    fn edge_weight(&self, id: Self::EdgeId) -> Option<&Self::EdgeWeight> {
        self.graph.edge_weight(id)
    }
}

impl<G: NodeIndexable> NodeIndexable for BitFiltered<G> {
    fn from_index(&self, i: usize) -> Self::NodeId {
        self.graph.from_index(i)
    }
    fn to_index(&self, a: Self::NodeId) -> usize {
        self.graph.to_index(a)
    }
    fn node_bound(&self) -> usize {
        self.graph.node_bound()
    }
}

impl<G: GetAdjacencyMatrix> GetAdjacencyMatrix for BitFiltered<G> {
    type AdjMatrix = G::AdjMatrix;

    fn adjacency_matrix(&self) -> Self::AdjMatrix {
        self.graph.adjacency_matrix()
    }
    fn is_adjacent(&self, matrix: &Self::AdjMatrix, a: Self::NodeId, b: Self::NodeId) -> bool {
        self.graph.is_adjacent(matrix, a, b)
    }
}

impl<'a, G: Data> IntoNodeIdentifiers for &'a BitFiltered<G>
where
    &'a G: IntoNodeIdentifiers<NodeId = G::NodeId>,
{
    type NodeIdentifiers = <&'a G as IntoNodeIdentifiers>::NodeIdentifiers;

    fn node_identifiers(self) -> Self::NodeIdentifiers {
        todo!()
    }
}

// impl<'a, G: Data> IntoNodeReferences for &'a BitFiltered<G>
// where
//     &'a G: IntoNodeReferences<NodeWeight = G::NodeWeight, NodeId = G::NodeId>,
// {
//     type NodeRef = <&'a G as IntoNodeReferences>::NodeRef;
//     type NodeReferences = <&'a G as IntoNodeReferences>::NodeReferences;

//     fn node_references(self) -> Self::NodeReferences {
//         self.graph.node_references()
//     }
// }

impl<'a, G: Data> IntoEdges for &'a BitFiltered<G>
where
    &'a G: IntoEdges<EdgeId = G::EdgeId, NodeId = G::NodeId, EdgeWeight = G::EdgeWeight>,
{
    type Edges = <&'a G as IntoEdges>::Edges;

    fn edges(self, a: Self::NodeId) -> Self::Edges {
        self.graph.edges(a)
    }
}

impl<'a, G: Data> IntoEdgeReferences for &'a BitFiltered<G>
where
    &'a G: IntoEdgeReferences<EdgeId = G::EdgeId, NodeId = G::NodeId, EdgeWeight = G::EdgeWeight>,
{
    type EdgeRef = <&'a G as IntoEdgeReferences>::EdgeRef;
    type EdgeReferences = <&'a G as IntoEdgeReferences>::EdgeReferences;

    fn edge_references(self) -> Self::EdgeReferences {
        self.graph.edge_references()
    }
}

impl<'a, G: Data> IntoNeighbors for &'a BitFiltered<G>
where
    &'a G: IntoNeighbors<EdgeId = G::EdgeId, NodeId = G::NodeId>,
{
    type Neighbors = <&'a G as IntoNeighbors>::Neighbors;

    fn neighbors(self, a: Self::NodeId) -> Self::Neighbors {
        self.graph.neighbors(a)
    }
}

impl<'a, G: Data> IntoNeighborsDirected for &'a BitFiltered<G>
where
    &'a G: IntoNeighborsDirected<EdgeId = G::EdgeId, NodeId = G::NodeId>,
{
    type NeighborsDirected = <&'a G as IntoNeighborsDirected>::NeighborsDirected;

    fn neighbors_directed(self, a: Self::NodeId, dir: Direction) -> Self::NeighborsDirected {
        self.graph.neighbors_directed(a, dir)
    }
}

impl<'a, G: Data> IntoEdgesDirected for &'a BitFiltered<G>
where
    &'a G: IntoEdgesDirected<EdgeId = G::EdgeId, NodeId = G::NodeId, EdgeWeight = G::EdgeWeight>,
{
    type EdgesDirected = <&'a G as IntoEdgesDirected>::EdgesDirected;

    fn edges_directed(self, a: Self::NodeId, dir: Direction) -> Self::EdgesDirected {
        self.graph.edges_directed(a, dir)
    }
}
