//! Creates and converts between
//!
//! - [`relative::Layout`], which is a logical description
//!   of how screens should be ordered, and
//! - [`absolute::Layout`], which is a physical description
//!   of at which pixel position and size each screen is
//!
//! See [`parse`] for a description of the format
//! which one can [`str::parse`] into [`relative::Layout`]
//!
//! Note: Conversion via [`relative::Layout::to_absolute`]
//! is not pure, but dependent on the currently running WM
//! to get screen resolutions and the works.
//!
//! Currently only support for Sway is implemented,
//! however, feel feel free to take a look inside [`comms`]
//! and open an issue or send a PR
//! if you'd like to add support for another WM!

pub mod absolute;
pub mod comms;
pub mod config;
pub mod convert;
pub mod geometry;
pub mod info;
pub mod parse;
pub mod relative;

use std::collections::BTreeMap;

use config::Config;
use eyre::{Context, ContextCompat, Result};

pub type Map<K, V> = BTreeMap<K, V>;

pub fn run() -> Result<()> {
    let config = Config::new()?;
    let desc = config
        .machine_layout()
        .context("Could not determine hostname to decide which layout to load")?
        .context("Config file does not define layout for this machine")?;

    let relative: relative::Layout = desc
        .parse()
        .context("Could not parse relative layout description")?;

    let mut comms = comms::establish().context("Could not establish connection to WM")?;
    let layout = relative
        .to_absolute(comms.as_mut())
        .context("Could not absolutize layout")?;
    comms
        .set_layout(&layout)
        .context("Could not set layout in WM")?;

    Ok(())
}
