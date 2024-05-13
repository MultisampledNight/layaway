use strum::{Display, EnumString};

use crate::{info::Resolution, Port};

/// Description of a screen layout,
/// based on relative positioning.
#[derive(Debug)]
pub struct RelativeLayout {
    pub screens: Vec<Screen>,
}

#[derive(Debug)]
pub struct Screen {
    pub port: Port,
    pub resolution: Option<Resolution>,
    pub scale: Option<f64>,
    pub pos: Position,
}

#[derive(Debug, Default)]
pub struct Position {
    pub hori: Horizontal,
    pub vert: Vertical,
}

#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString,
)]
pub enum Horizontal {
    Left,
    Center,
    #[default]
    Right,
}

#[derive(
    Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString,
)]
pub enum Vertical {
    #[default]
    Top,
    Horizon,
    Bottom,
}
