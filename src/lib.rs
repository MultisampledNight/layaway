pub mod absolute;
pub mod geometry;
pub mod info;
pub mod parse;
pub mod relative;

use std::fmt;

use eyre::Result;
use info::Connector;
use relative::RelativeLayout;

pub fn run() -> Result<()> {
    let layout = "hdmi@1200p + edp/c,b";
    let layout = layout.parse::<RelativeLayout>();
    dbg!(layout);
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
