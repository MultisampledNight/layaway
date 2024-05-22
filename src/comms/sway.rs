use std::num::ParseIntError;

use swayipc::Connection;
use thiserror::Error;

use crate::{
    absolute::{self, Output, OutputConfig, OutputRef},
    geometry::{Interval, Rect},
};

use super::{Port, Result};

pub fn establish() -> Result<Box<dyn super::Comms>> {
    let conn = Connection::new().map_err(Error::SwayIpc)?;
    Ok(Box::new(Comms { conn }) as Box<dyn super::Comms>)
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

    fn set_layout(&mut self, layout: &absolute::Layout) -> Result<()> {
        for cmd in layout.to_sway_commands() {
            self.conn
                .run_command(cmd)
                // all below is just propagating errors, if any
                .map_err(Error::SwayIpc)?
                .into_iter()
                .collect::<Result<(), _>>()
                .map_err(Error::SwayIpc)?;
        }

        Ok(())
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

impl absolute::Layout {
    pub fn to_sway_commands(&self) -> impl Iterator<Item = String> + '_ {
        self.outputs().map(|output| output.to_sway_command())
    }
}

impl OutputRef<'_> {
    #[must_use]
    pub fn to_sway_command(&self) -> String {
        let bounds = self.cfg.bounds;
        let size = bounds.size();
        format!(
            concat!(
                "output {port} ",
                "position {pos_x} {pos_y} ",
                "resolution {res_width}x{res_height} ",
                "scale {scale}",
            ),
            port = self.port,
            pos_x = bounds.x.start(),
            pos_y = bounds.y.start(),
            res_width = size.width,
            res_height = size.height,
            scale = self.cfg.scale,
        )
    }
}
