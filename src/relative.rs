use std::fmt;

use crate::{comms::Port, info::Resolution};

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
    pub pos: Position,
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hori {
    Left,
    #[default]
    Right,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Vert {
    #[default]
    Top,
    Bottom,
}

pub type HoriSpec = MaybeCenter<Hori>;
pub type VertSpec = MaybeCenter<Vert>;

impl Default for HoriSpec {
    fn default() -> Self {
        Self::Center
    }
}

impl Default for VertSpec {
    fn default() -> Self {
        Self::Extreme(Vert::Top)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MaybeCenter<T: Clone + Copy + fmt::Debug> {
    Extreme(T),
    Center,
}

impl<T: Clone + Copy + fmt::Debug> MaybeCenter<T> {
    pub fn map<U: Clone + Copy + fmt::Debug>(self, op: impl FnOnce(T) -> U) -> MaybeCenter<U> {
        match self {
            Self::Center => MaybeCenter::Center,
            Self::Extreme(extreme) => MaybeCenter::Extreme(op(extreme)),
        }
    }
}

impl<T: Clone + Copy + fmt::Debug> From<T> for MaybeCenter<T> {
    fn from(value: T) -> Self {
        Self::Extreme(value)
    }
}
