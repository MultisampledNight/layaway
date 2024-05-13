pub mod geometry;

use std::collections::BTreeMap;

use eyre::Result;
use geometry::Rect;
use strum::{Display, EnumString};

pub fn run() -> Result<()> {
    Ok(())
}

/// Where each [`Output`] concretely is, pixel-wise.
pub struct ConcreteLayout {
    pub outputs: BTreeMap<Port, Rect>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Output {
    port: Port,
    dims: Rect,
}

/// Where an [`Output`] is plugged in.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port {
    pub kind: Connector,
    pub idx: u32,
}

/// Names taken from:
///
/// - https://en.wikipedia.org/wiki/DisplayPort
/// - https://en.wikipedia.org/wiki/Mobile_High-Definition_Link
/// - https://hdmi.org
///
/// The actual names how sway probably wants them are mostly guessed,
/// can't be bothered to actually look it up.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum Connector {
    /// DisplayPort.
    Dp,
    /// Mini DisplayPort.
    Mdp,
    /// Embedded DisplayPort.
    Edp,
    /// Internal DisplayPort.
    Idp,
    /// Portable Digital Media Interface.
    Pdmi,
    /// Wireless DisplayPort.
    Wdp,

    /// High-Definition Multimedia Interface®.
    Hdmi,
    HdmiA,

    /// Low-voltage differential signaling.
    /// Common on old laptops.
    Lvds,

    Dvi,
    Vga,
    Scart,
}
