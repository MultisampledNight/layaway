use std::collections::BTreeMap;

use crate::{geometry::Rect, Port};

/// How each output should be configured,
/// as seen from Sway.
pub struct AbstractLayout {
    pub outputs: BTreeMap<Port, OutputConfig>,
}

impl AbstractLayout {
    pub fn outputs(&self) -> impl Iterator<Item = OutputRef<'_>> {
        self.outputs
            .iter()
            .map(|(port, cfg)| OutputRef { port, cfg })
    }
}

/// Something that Sway can display to. Usually a screen.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct OutputRef<'layout> {
    /// Where this output is physically connected.
    port: &'layout Port,
    /// Properties of this output, like size and scale.
    cfg: &'layout OutputConfig,
}

/// Configuration for a given output in sway.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct OutputConfig {
    /// Where this output is placed in Sway.
    dims: Rect,
    /// With what size multiplier to have Wayland applications rendered.
    /// if they are visible on this output.
    scale: f64,
}
