use crate::{comms::Port, geometry::Rect, Map};

/// How each output should be configured,
/// as seen from the WM.
#[derive(Debug)]
pub struct Layout {
    pub outputs: Map<Port, OutputConfig>,
}

impl Layout {
    pub fn outputs(&self) -> impl Iterator<Item = OutputRef<'_>> {
        self.outputs
            .iter()
            .map(|(port, cfg)| OutputRef { port, cfg })
    }
}

impl FromIterator<Output> for Layout {
    fn from_iter<I: IntoIterator<Item = Output>>(iter: I) -> Self {
        let outputs = iter
            .into_iter()
            .map(|Output { port, cfg }| (port, cfg))
            .collect();

        Self { outputs }
    }
}

/// Something that the WM can display to. Usually a screen.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Output {
    /// Where this output is physically connected.
    pub port: Port,
    /// Properties of this output, like position and scale.
    pub cfg: OutputConfig,
}

/// Version of [`Output`] where every field is a reference.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct OutputRef<'layout> {
    pub port: &'layout Port,
    pub cfg: &'layout OutputConfig,
}

impl From<OutputRef<'_>> for Output {
    fn from(OutputRef { port, cfg }: OutputRef<'_>) -> Self {
        Self {
            port: port.to_owned(),
            cfg: cfg.to_owned(),
        }
    }
}

impl<'a> From<&'a Output> for OutputRef<'a> {
    fn from(Output { ref port, ref cfg }: &'a Output) -> Self {
        Self { port, cfg }
    }
}

/// Configuration for a given output in the WM.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct OutputConfig {
    /// Where this output is placed in the WM.
    pub bounds: Rect,
    /// With what size multiplier to have applications rendered
    /// if they are visible on this output.
    pub scale: f64,
    /// If the output is currently on and displaying.
    pub active: bool,
}
