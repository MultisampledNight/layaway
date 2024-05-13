use strum::{Display, EnumString};

use crate::{geometry::Size, info::Connector};

/// Description of a screen layout,
/// based on relative positioning.
#[derive(Debug)]
pub struct RelativeLayout {
    pub screens: Vec<Screen>,
}

#[derive(Debug)]
pub struct Screen {
    pub connector: Connector,
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
    #[strum(serialize = "left")]
    Left,
    #[strum(serialize = "center")]
    Center,
    #[strum(serialize = "right")]
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum Vertical {
    #[strum(serialize = "top")]
    Top,
    #[strum(serialize = "horizon")]
    Horizon,
    #[strum(serialize = "right")]
    Bottom,
}
