use crate::{
    comms::Port,
    geometry::{Hori, MaybeCenter, Rotation, Vert},
    info::Resolution,
};

/// Description of a screen layout,
/// based on relative positioning.
#[derive(Debug)]
pub struct Layout {
    pub screens: Vec<Screen>,
}

#[derive(Debug)]
pub struct Screen {
    pub port: Port,
    pub resolution: Option<Resolution>,
    pub scale: Option<f64>,
    pub transform: Transform,
    pub pos: Position,
}

#[derive(Debug, Default)]
pub struct Transform {
    pub flipped: bool,
    pub rotation: Rotation,
}

#[derive(Debug)]
pub enum Position {
    Hori { edge: Hori, spec: MaybeCenter<Vert> },
    Vert { edge: Vert, spec: MaybeCenter<Hori> },
}

impl Default for Position {
    fn default() -> Self {
        Self::Hori {
            edge: Hori::default(),
            spec: MaybeCenter::Extreme(Vert::Top),
        }
    }
}
