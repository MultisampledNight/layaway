use std::num::ParseIntError;

use swayipc::Connection;
use thiserror::Error;

use crate::{
    absolute::{self, Output, OutputConfig},
    geometry::{Interval, Rect},
};

use super::{BoxComms, Port, Result};

pub fn establish() -> Result<BoxComms> {
    let conn = Connection::new().map_err(Error::SwayIpc)?;
    Ok(Box::new(Comms { conn }) as BoxComms)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Over IPC: {0}")]
    SwayIpc(#[from] swayipc::Error),
    #[error("Could not parse output name into port: {0}")]
    ParsePort(ParsePortError),
}

#[derive(Debug)]
pub struct Comms {
    pub conn: Connection,
}

impl super::Comms for Comms {
    fn layout(&mut self) -> Result<absolute::Layout> {
        let outputs = self.conn.get_outputs().map_err(Error::SwayIpc)?;
        let layout = outputs
            .into_iter()
            .map(Output::try_from)
            .collect::<Result<absolute::Layout, ParsePortError>>()
            .map_err(Error::ParsePort)?;

        Ok(layout)
    }

    fn set_layout(&mut self, layout: absolute::Layout) -> Result<()> {
        todo!()
    }
}

impl TryFrom<swayipc::Output> for Output {
    type Error = ParsePortError;
    fn try_from(raw: swayipc::Output) -> Result<Self, ParsePortError> {
        Ok(Self {
            port: Port::parse_from_sway(&raw.name)?,
            cfg: OutputConfig {
                bounds: raw.rect.into(),
                scale: raw.scale.unwrap_or(1.0),
                active: raw.active,
            },
        })
    }
}

impl Port {
    fn parse_from_sway(name: &str) -> Result<Self, ParsePortError> {
        let (kind, idx) = name
            .rsplit_once('-')
            .ok_or_else(|| ParsePortError::NoDash {
                name: name.to_string(),
            })?;

        Ok(Self {
            kind: kind.parse().map_err(|_| ParsePortError::NewConnector {
                connector: kind.to_string(),
            })?,
            idx: idx.parse().map_err(|err| ParsePortError::IdxNotANumber {
                idx: idx.to_string(),
                err,
            })?,
        })
    }
}

#[derive(Debug, Error)]
pub enum ParsePortError {
    #[error("Output name must contain a dash to separate connector from index, but is `{name}`")]
    NoDash { name: String },
    #[error("New unknown connector name `{connector}`, perhaps libDRM got updated with new connectors? Need to add them in source here then. Feel free to report this!")]
    NewConnector { connector: String },
    #[error("Port index `{idx}` is not an integer: {err}")]
    IdxNotANumber { idx: String, err: ParseIntError },
}

impl From<swayipc::Rect> for Rect {
    fn from(model: swayipc::Rect) -> Self {
        Self {
            x: Interval::new(model.x, model.x + model.width),
            y: Interval::new(model.y, model.y + model.height),
        }
    }
}
