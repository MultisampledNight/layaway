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
    dbg!(comms.layout());
    let layout: relative::Layout = "hdmia@1200p + edp/bottom,center".parse()?;
    //let layout: absolute::Layout = layout.into();
    let _ = dbg!(layout);
    Ok(())
}
