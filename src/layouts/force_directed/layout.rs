#![allow(clippy::similar_names)]
#![allow(clippy::type_complexity)]

use egui::{Pos2, Vec2};
use fdg::{
    fruchterman_reingold::{FruchtermanReingold, FruchtermanReingoldConfiguration},
    nalgebra::{clamp, OPoint, SVector},
    Force, ForceGraph,
};
use petgraph::{
    csr::{DefaultIx, IndexType},
    visit::IntoNodeReferences,
    Directed, EdgeType,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    layouts::{Layout, LayoutState},
    DisplayEdge, DisplayNode, Edge, Graph, Node,
};

const DT: f32 = 0.05;
const GRAVITY: f32 = 3.0;
const EPSILON: f32 = 0.001;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State<const D: usize = 2, Ix: IndexType = DefaultIx> {
    is_running: bool,
    algo: FruchtermanReingold<f32, D, Ix>,
    v: f32,
}

impl<const D: usize, Ix: IndexType> LayoutState for State<D, Ix> where
    Ix: DeserializeOwned + Serialize + Send + Sync
{
}

impl<const D: usize, Ix: IndexType> Default for State<D, Ix> {
    fn default() -> Self {
        State {
            is_running: true,
            algo: FruchtermanReingold {
                conf: FruchtermanReingoldConfiguration {
                    dt: 0.02,
                    cooloff_factor: 0.99,
                    scale: 300.0,
                },
                velocities: Default::default(),
            },
            v: f32::INFINITY,
        }
    }
}

impl<N, E, Ty, Ix, Dn, De> Layout<State<2, Ix>, N, E, Ty, Ix, Dn, De> for State<2, Ix>
where
    N: Clone,
    E: Clone,
    Ty: EdgeType + Clone,
    Ix: DeserializeOwned + Serialize + Send + Sync + IndexType,
    Dn: DisplayNode<N, E, Ty, Ix>,
    De: DisplayEdge<N, E, Ty, Ix, Dn>,
{
    fn from_state(state: State<2, Ix>) -> impl Layout<State<2, Ix>, N, E, Ty, Ix, Dn, De> {
        state
    }

    fn next(&mut self, g: &mut Graph<N, E, Ty, Ix, Dn, De>)
    where
        N: Clone,
        E: Clone,
        Ty: EdgeType + Clone,
        Ix: IndexType + Clone,
        Dn: DisplayNode<N, E, Ty, Ix>,
        De: DisplayEdge<N, E, Ty, Ix, Dn>,
    {
        if !self.is_running || g.node_count() == 0 {
            return;
        }

        let gx: &mut ForceGraph<
            f32,
            2,
            Node<N, E, Ty, Ix, Dn>,
            Edge<N, E, Ty, Ix, Dn, De>,
            Ty,
            Ix,
        > = g.g_mut();
        #[allow(clippy::unreadable_literal)]
        let coeff: usize = clamp(((self.v / 0.00000000000002).round() as usize), 0, 30);

        self.algo.apply_many(gx, coeff);
        let mut vavg: SVector<_, 2> = Default::default();
        for v in self.algo.velocities.values() {
            vavg += v / self.algo.velocities.len() as f32;
        }
        self.v = vavg.norm();

        for (n, p) in g.g_mut().node_weights_mut() {
            let p = Pos2::new(p.x, p.y);
            n.set_location(p);
        }
    }
    fn state(&self) -> State<2, Ix> {
        self.clone()
    }
}
