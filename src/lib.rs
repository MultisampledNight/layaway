pub mod absolute;
pub mod comms;
pub mod convert;
pub mod geometry;
pub mod info;
pub mod parse;
pub mod relative;

use std::collections::BTreeMap;

use eyre::Result;

pub type Map<K, V> = BTreeMap<K, V>;

pub fn run() -> Result<()> {
    let mut comms = comms::establish()?;
    let layout: relative::Layout = "dp3 + hdmia1/right,bottom".parse()?;
    let layout = layout.to_absolute(comms.as_mut())?;
    let _ = dbg!(layout);
    Ok(())
}
