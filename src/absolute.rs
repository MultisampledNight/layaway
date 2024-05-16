use std::collections::BTreeMap;

use crate::{comms::Port, geometry::Rect};

/// How each output should be configured,
/// as seen from Sway.
pub struct Layout {
    pub outputs: BTreeMap<Port, OutputConfig>,
}

impl Layout {
    pub fn outputs(&self) -> impl Iterator<Item = OutputRef<'_>> {
        self.outputs
            .iter()
            .map(|(port, cfg)| OutputRef { port, cfg })
    }
}

pub struct Output {}

/// Something that Sway can display to. Usually a screen.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct OutputRef<'layout> {
    /// Where this output is physically connected.
    port: &'layout Port,
    /// Properties of this output, like position and scale.
    cfg: &'layout OutputConfig,
}

/// Configuration for a given output in sway.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct OutputConfig {
    /// Where this output is placed in the WM.
    bounds: Rect,
    /// With what size multiplier to have applications rendered
    /// if they are visible on this output.
    scale: f64,
}
