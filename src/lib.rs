pub mod absolute;
pub mod geometry;
pub mod relative;

use std::fmt;

use eyre::Result;
use strum::{Display, EnumString};

pub fn run() -> Result<()> {
    Ok(())
}

/// Where an output is plugged in.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port {
    pub kind: Connector,
    pub idx: u32,
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.kind, self.idx)
    }
}

/// Protocol and possibly physical form of the cable/plug
/// used to connect an output to the system.
///
/// Names taken from:
///
/// - <https://en.wikipedia.org/wiki/DisplayPort>
/// - <https://en.wikipedia.org/wiki/Mobile_High-Definition_Link>
/// - <https://hdmi.org>
///
/// The actual names how sway probably wants them are mostly guessed,
/// can't be bothered to actually look it up.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
pub enum Connector {
    /// DisplayPort.
    #[strum(serialize = "DP")]
    Dp,
    /// Mini DisplayPort.
    #[strum(serialize = "mDP")]
    Mdp,
    /// Embedded DisplayPort.
    #[strum(serialize = "eDP")]
    Edp,
    /// Internal DisplayPort.
    #[strum(serialize = "iDP")]
    Idp,
    /// Portable Digital Media Interface.
    #[strum(serialize = "PDMI")]
    Pdmi,
    /// Wireless DisplayPort.
    #[strum(serialize = "wDP")]
    Wdp,

    /// High-Definition Multimedia Interface®.
    #[strum(serialize = "HDMI")]
    Hdmi,
    // not sure what's the difference to normal hdmi
    // on all machines only this one is found though
    #[strum(serialize = "HDMI-A")]
    HdmiA,

    /// Low-voltage differential signaling.
    /// Common on old laptops.
    #[strum(serialize = "LVDS")]
    Lvds,

    #[strum(serialize = "DVI")]
    Dvi,
    #[strum(serialize = "VGA")]
    Vga,
    #[strum(serialize = "SCART")]
    Scart,
}
