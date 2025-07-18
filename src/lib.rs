#![allow(clippy::default_trait_access)]

pub mod draw;
pub mod elements;
pub mod graph;
pub mod graph_view;
mod helpers;
mod layouts;
pub mod metadata;
mod settings;

pub use draw::{DefaultEdgeShape, DefaultNodeShape, DisplayEdge, DisplayNode, DrawContext};
pub use elements::{Edge, EdgeProps, Node, NodeProps};
pub use graph::Graph;
pub use graph_view::{DefaultGraphView, GraphView};

#[allow(deprecated)]
pub use helpers::{
    add_edge, add_edge_custom, add_node, add_node_custom, default_edge_transform,
    default_node_transform, generate_random_graph, generate_simple_digraph,
    generate_simple_ungraph, node_size, to_graph, to_graph_custom,
};

pub use layouts::force_directed::State as LayoutForce;

pub use metadata::Metadata;
pub use settings::{SettingsInteraction, SettingsNavigation, SettingsStyle};

#[cfg(feature = "events")]
pub mod events;

pub use graph::new_from_raw;
