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

use clap::{ArgAction, Parser};
use config::{Config, LayoutDesc};
use eyre::{Context, ContextCompat, Result};

pub type Map<K, V> = BTreeMap<K, V>;

/// Calculates the physical screen layout given a short relative layout description.
#[derive(Debug, Parser)]
pub struct Args {
    #[allow(rustdoc::bare_urls)]
    /// Instead of using the machine-specific layout description from the config file,
    /// use the given layout description.
    ///
    /// See the README at https://github.com/MultisampledNight/layaway
    /// for details on the format.
    ///
    /// By default, the config file (`~/.config/layaway/config.toml` on Linux in most cases)
    /// is used to look up the layout description for the given hostname.
    desc: Option<LayoutDesc>,

    /// Instead of applying the calculated layout,
    /// print the corresponding WM configuration to stdout.
    ///
    /// By default, the calculated layout is directly applied to the WM,
    /// so that it becomes effective.
    #[arg(short = 'n', long = "no-apply", action = ArgAction::SetFalse)]
    apply: bool,
}

pub fn run() -> Result<()> {
    let args = Args::parse();

    let desc = args.desc.map_or_else(desc_from_config, Ok)?;

    let relative: relative::Layout = desc
        .parse()
        .context("Could not parse relative layout description")?;

    let mut comms = comms::establish().context("Could not establish connection to WM")?;
    let layout = relative
        .to_absolute(comms.as_mut())
        .context("Could not absolutize layout")?;

    if args.apply {
        comms
            .set_layout(&layout)
            .context("Could not set layout in WM")?;
    } else {
        for cmd in layout.to_sway_commands() {
            println!("{cmd}");
        }
    }

    Ok(())
}

pub fn desc_from_config() -> Result<LayoutDesc> {
    let config = Config::new()?;
    let desc = config
        .machine_layout()
        .context("Could not determine hostname to decide which layout to load")?
        .context("Config file does not define layout for this machine")?;
    Ok(desc.to_string())
}
