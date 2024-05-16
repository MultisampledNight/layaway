use std::fmt;

use crate::{absolute, info::Connector, Map};

pub type Name = String;

/// Communicates with the window manager,
/// in order to fetch information about available outputs.
pub trait Comms {
    fn outputs() -> Map<Name, absolute::Output>;
}

/// Where an output is plugged in.
///
/// This is heavily biased towards how Sway on DRM handles displays.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port {
    pub kind: Connector,
    pub idx: u32,
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.kind, self.idx)
    }
}
