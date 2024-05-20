//! Concretizes [`relative::Layout`] into [`absolute::Layout`]

use crate::{
    absolute,
    comms::{self, Comms},
    geometry::{Interval, Pixel, Rect},
    relative::{self, Hori, HoriSpec, Position, Vert, VertSpec},
};

impl relative::Layout {
    /// Resolve the layout according to the currently connected displays.
    pub fn to_absolute(&self, comms: &mut dyn Comms) -> comms::Result<absolute::Layout> {
        let mut placed = absolute::Layout::new();
        let currently_active = comms.layout()?;
        let mut bbox = Rect {
            x: Interval::new(0, 0),
            y: Interval::new(0, 0),
        };

        for screen in &self.screens {
            let screen_size = screen.resolution.map(|res| res.size()).or_else(|| {
                currently_active
                    .outputs
                    .get(&screen.port)
                    .map(|cfg| cfg.bounds.size())
            });
            let Some(screen_size) = screen_size else {
                // user specified screen that isn't connected
                // hence should not affect layout
                continue;
            };

            let bounds = match screen.pos {
                Position::Hori { edge, spec } => Rect {
                    x: bbox.x.place_outside(screen_size.width, edge.into()),
                    y: bbox.y.place_inside(screen_size.height, spec.into()),
                },
                Position::Vert { edge, spec } => Rect {
                    x: bbox.x.place_inside(screen_size.width, spec.into()),
                    y: bbox.y.place_outside(screen_size.height, edge.into()),
                },
            };

            // now that we've got the screen bounds, make sure it's actually noticed
            // by the bounding box
            // so future screens can be placed accordingly
            bbox.stretch_to_rect(bounds);

            // that'd be it! let's actually place the screen
            // we just calculated its bounds of
            let scale = screen.scale.unwrap_or({
                if screen_size.height > 4000 {
                    2.0
                } else {
                    1.0
                }
            });
            placed.add(absolute::Output {
                port: screen.port,
                cfg: absolute::OutputConfig {
                    bounds,
                    scale,
                    active: true,
                },
            });
        }

        Ok(placed)
    }
}

impl Interval {
    /// Creates a new [`Interval`] of the given `length` next to this interval,
    /// on the given `side`.
    /// The new interval will touch this one and share one limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use layaway::{convert::Extreme, geometry::Interval};
    /// let space = Interval::new(100, 200);
    /// let length = 20;
    /// assert_eq!(
    ///     space.place_adjacent(10, Extreme::Least),
    ///     Interval::new(90, 100),
    /// );
    /// ```
    pub fn place_outside(self, length: Pixel, side: Extreme) -> Self {
        match side {
            Extreme::Least => Self::new(self.start() - length, self.start()),
            Extreme::Most => Self::new(self.end(), self.end() + length),
        }
    }

    /// Creates a new [`Interval`] of the given `length` inside of interval,
    /// on the given `side`.
    pub fn place_inside(self, length: Pixel, pos: ExtremeOrCenter) -> Self {
        match pos {
            ExtremeOrCenter::Least => Self::new(self.start(), self.start() + length),
            ExtremeOrCenter::Center => Self::new(self.mid() - length / 2, self.mid() + length / 2),
            ExtremeOrCenter::Most => Self::new(self.end() - length, self.end()),
        }
    }
}

/// Specifies one side of a 1D [`Interval`].
#[derive(Clone, Copy, Debug)]
pub enum Extreme {
    Least,
    Most,
}

/// Specifies one side or the center of a 1D [`Interval`].
#[derive(Clone, Copy, Debug)]
pub enum ExtremeOrCenter {
    Least,
    Center,
    Most,
}

// assuming a x+ right, y- bottom coordinate system

impl From<Hori> for Extreme {
    fn from(value: Hori) -> Self {
        match value {
            Hori::Left => Self::Least,
            Hori::Right => Self::Most,
        }
    }
}

impl From<Vert> for Extreme {
    fn from(value: Vert) -> Self {
        match value {
            Vert::Top => Self::Least,
            Vert::Bottom => Self::Most,
        }
    }
}

impl From<HoriSpec> for ExtremeOrCenter {
    fn from(value: HoriSpec) -> Self {
        match value {
            HoriSpec::Left => Self::Least,
            HoriSpec::Center => Self::Center,
            HoriSpec::Right => Self::Most,
        }
    }
}

impl From<VertSpec> for ExtremeOrCenter {
    fn from(value: VertSpec) -> Self {
        match value {
            VertSpec::Top => Self::Least,
            VertSpec::Center => Self::Center,
            VertSpec::Bottom => Self::Most,
        }
    }
}
