use std::{fmt::Write, num::ParseIntError};

use swayipc::Connection;
use thiserror::Error;

use crate::{
    absolute::{self, Output, OutputConfig, OutputRef},
    geometry::{Interval, Rect, Rotation, Size, Transform},
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
    #[error("Could not parse output name `{raw}` into port: {err}")]
    ParsePort { raw: String, err: ParsePortError },
    #[error("Could not parse transform `{raw}`: {err}")]
    ParseTransform {
        raw: String,
        err: ParseTransformError,
    },
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
            .collect::<Result<absolute::Layout, Error>>()?;

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
    type Error = Error;
    fn try_from(raw: swayipc::Output) -> Result<Self, Self::Error> {
        Ok(Self {
            port: Port::parse_from_sway(&raw.name).map_err(|err| Error::ParsePort {
                raw: raw.name.clone(),
                err,
            })?,
            cfg: OutputConfig {
                bounds: raw.rect.into(),
                resolution: raw.current_mode.map(Into::into),
                scale: raw.scale.unwrap_or(1.0),
                transform: raw.transform.map_or(Ok(Transform::default()), |raw| {
                    Transform::parse_from_sway(&raw).map_err(|err| Error::ParseTransform {
                        raw: raw.to_string(),
                        err,
                    })
                })?,
                active: raw.active,
            },
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
pub enum ParseTransformError {
    #[error("Angle `{raw}` could not be parsed, was none of normal (only if not flipped), 90, 180 or 270")]
    InvalidAngle { raw: String },
}

impl Transform {
    pub fn parse_from_sway(raw: &str) -> Result<Self, ParseTransformError> {
        let flipped = raw.contains("flipped");

        let angle = if let Some(angle) = raw.strip_prefix("flipped-") {
            angle
        } else if raw == "normal" || raw == "flipped" {
            "0"
        } else {
            // assume no flip but still rotation
            raw
        };

        let rotation = match angle {
            "0" => Rotation::None,
            "90" => Rotation::Quarter,
            "180" => Rotation::Half,
            "270" => Rotation::ThreeQuarter,
            _ => {
                return Err(ParseTransformError::InvalidAngle {
                    raw: raw.to_string(),
                })
            }
        };

        Ok(Self { flipped, rotation })
    }

    #[must_use]
    pub fn to_sway(&self) -> String {
        if !self.flipped && matches!(self.rotation, Rotation::None) {
            return "normal".to_string();
        }

        let mut parts = Vec::new();

        if self.flipped {
            parts.push("flipped");
        }

        match self.rotation {
            Rotation::None => (),
            Rotation::Quarter => parts.push("90"),
            Rotation::Half => parts.push("180"),
            Rotation::ThreeQuarter => parts.push("270"),
        };

        parts.join("-")
    }
}

impl From<swayipc::Rect> for Rect {
    fn from(
        swayipc::Rect {
            x,
            y,
            width,
            height,
            ..
        }: swayipc::Rect,
    ) -> Self {
        Self {
            x: Interval::new(x, x + width),
            y: Interval::new(y, y + height),
        }
    }
}

impl From<swayipc::Mode> for Size {
    fn from(swayipc::Mode { width, height, .. }: swayipc::Mode) -> Self {
        Self { width, height }
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
        let OutputConfig {
            bounds,
            resolution,
            scale,
            ..
        } = self.cfg;

        let mut cmd = format!(
            concat!(
                "output {port} ",
                "position {pos_x} {pos_y} ",
                "scale {scale} ",
                "transform {transform}",
            ),
            port = self.port,
            pos_x = bounds.x.start(),
            pos_y = bounds.y.start(),
            scale = scale,
            transform = self.cfg.transform.to_sway(),
        );

        if let Some(res) = resolution {
            write!(cmd, " resolution {}x{}", res.width, res.height).unwrap();
        }

        cmd
    }
}
