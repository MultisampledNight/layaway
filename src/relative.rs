use strum::{Display, EnumString};

use crate::{geometry::Size, Port};

/// Description of a screen layout,
/// based on relative positioning.
#[derive(Debug)]
pub struct RelativeLayout {
    pub screens: Vec<Screen>,
}

#[derive(Debug)]
pub struct Screen {
    pub port: Port,
    pub resolution: Option<Size>,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Position {
    pub hori: Horizontal,
    pub vert: Vertical,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum Horizontal {
    Left,
    Center,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum Vertical {
    Top,
    Horizon,
    Bottom,
}
