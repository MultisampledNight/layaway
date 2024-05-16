//! Communication with the window manager (WM)
//! to learn about available screens
//! and apply the calculated ones.
//!
//! Only comms with [Sway](https://swaywm.org/) via [`swayipc`] are implemented.
//! Support for other WMs can be added via:
//!
//! 1. Adding a new submodule named after the WM, henceforth called `a`
//! 2. Adding a struct in `a` that implements [`Comms`]
//! 3. Building that struct in [`establish`]
//!    if there are signs present that the WM is running
//!    in the current session

pub mod sway;

use std::{env, fmt};

use thiserror::Error;

use crate::{absolute, info::Connector};

pub type Name = String;

/// Figure out what WM we're running on and
pub fn establish() -> Result<BoxComms, Error> {
    let comms = if env::var("SWAYSOCK").is_ok() {
        sway::establish()?
    } else {
        return Err(Error::NoWmRunning);
    };

    Ok(comms)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("When communicating with sway: {0}")]
    Sway(#[from] sway::Error),
    #[error("No known WM is running")]
    NoWmRunning,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Communicates with the window manager,
/// in order to fetch information about available outputs.
pub trait Comms {
    fn layout(&mut self) -> Result<absolute::Layout>;
    fn set_layout(&mut self, layout: absolute::Layout) -> Result<()>;
}

pub type BoxComms = Box<dyn Comms>;

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
