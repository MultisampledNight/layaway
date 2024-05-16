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
    Hori { edge: Hori, spec: VertSpec },
    Vert { edge: Vert, spec: HoriSpec },
}

impl Default for Position {
    fn default() -> Self {
        Self::Hori {
            edge: Hori::default(),
            spec: VertSpec::default(),
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

// generics don't make this much nicer, difficult to name

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HoriSpec {
    Left,
    #[default]
    Center,
    Right,
}

impl From<Hori> for HoriSpec {
    fn from(hori: Hori) -> Self {
        match hori {
            Hori::Left => Self::Left,
            Hori::Right => Self::Right,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VertSpec {
    #[default]
    Top,
    Center,
    Bottom,
}

impl From<Vert> for VertSpec {
    fn from(hori: Vert) -> Self {
        match hori {
            Vert::Top => Self::Top,
            Vert::Bottom => Self::Bottom,
        }
    }
}
