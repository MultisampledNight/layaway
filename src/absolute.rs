use crate::{
    comms::Port,
    geometry::{Point, Rect, Size, Transform},
    Map,
};

/// How each output should be configured,
/// as seen from the WM.
#[derive(Debug, Default)]
pub struct Layout {
    pub outputs: Map<Port, OutputConfig>,
}

impl Layout {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn outputs(&self) -> impl Iterator<Item = OutputRef<'_>> {
        self.outputs
            .iter()
            .map(|(port, cfg)| OutputRef { port, cfg })
    }

    pub fn add(&mut self, output: Output) {
        self.outputs.insert(output.port, output.cfg);
    }

    /// The smallest rectangle that includes all output bounds.
    pub fn bounding_box(&self) -> Rect {
        let mut bb = Rect::default();
        for cfg in self.outputs.values() {
            bb.stretch_to_rect(cfg.bounds);
        }
        bb
    }

    /// Move all outputs so that the bounding box has a corner at the origin.
    /// Their relative positions to each other aren't changed.
    ///
    /// This implies moving all bounds into the positive space.
    /// Some applications appear to only use unsigned numbers
    /// for their absolute positions,
    /// so this might fix their inputs.
    pub fn reset_to_origin(&mut self) {
        // find out how much we need to move
        let bb = self.bounding_box();
        let least = Point {
            x: bb.x.start(),
            y: bb.y.start(),
        };

        // then actually do move everything
        for cfg in self.outputs.values_mut() {
            cfg.bounds -= least;
        }
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

    /// Unscaled physical resolution of the screen.
    /// [`None`] if the screen is not active.
    pub resolution: Option<Size>,

    /// With what size multiplier to have applications rendered
    /// if they are visible on this output.
    pub scale: f64,

    /// How the output is flipped and rotated.
    pub transform: Transform,

    /// If the output is currently on and displaying.
    pub active: bool,
}
