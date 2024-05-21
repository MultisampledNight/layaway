//! Concretizes [`relative::Layout`] into [`absolute::Layout`]

use crate::{
    absolute,
    comms::{self, Comms},
    geometry::{Interval, Pixel, Rect},
    relative::{self, Hori, MaybeCenter, Position, Vert},
};

impl relative::Layout {
    /// Resolve the layout according to the currently connected displays.
    pub fn to_absolute(&self, comms: &mut dyn Comms) -> comms::Result<absolute::Layout> {
        let mut placed = absolute::Layout::new();
        let currently_active = comms.layout()?;
        let mut bb = Rect {
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

            // note: order of x/y placement does not actually matter
            // they don't have any influence on each other
            let bounds = match screen.pos {
                // place left/right of bbox, then decide exact vertical placement
                Position::Hori { edge, spec } => Rect {
                    x: bb.x.place_outside(screen_size.width, edge.into()),
                    y: bb.y.place_inside(screen_size.height, spec.map(Into::into)),
                },
                // place top/bottom of bbox, then decide exact horizontal placement
                Position::Vert { edge, spec } => Rect {
                    x: bb.x.place_inside(screen_size.width, spec.map(Into::into)),
                    y: bb.y.place_outside(screen_size.height, edge.into()),
                },
            };

            // now that we've got the screen bounds, make sure it's actually noticed
            // by the bounding box
            // so future screens can be placed accordingly
            bb.stretch_to_rect(bounds);

            // that'd be it! let's actually place the output screen
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
    /// # use layaway::{convert::Side, geometry::Interval};
    /// let space = Interval::new(100, 200);
    /// let length = 20;
    /// assert_eq!(
    ///     space.place_adjacent(10, Side::Least),
    ///     Interval::new(90, 100),
    /// );
    /// ```
    pub fn place_outside(self, length: Pixel, side: Side) -> Self {
        match side {
            Side::Least => Self::new(self.start() - length, self.start()),
            Side::Most => Self::new(self.end(), self.end() + length),
        }
    }

    /// Creates a new [`Interval`] of the given `length` inside of interval,
    /// on the given `side`.
    pub fn place_inside(self, length: Pixel, pos: MaybeCenter<Side>) -> Self {
        match pos {
            MaybeCenter::Extreme(Side::Least) => Self::new(self.start(), self.start() + length),
            MaybeCenter::Center => Self::new(self.mid() - length / 2, self.mid() + length / 2),
            MaybeCenter::Extreme(Side::Most) => Self::new(self.end() - length, self.end()),
        }
    }
}

/// Specifies one side of a 1D [`Interval`].
#[derive(Clone, Copy, Debug)]
pub enum Side {
    Least,
    Most,
}

// assuming a x+ right, y- bottom coordinate system

impl From<Hori> for Side {
    fn from(value: Hori) -> Self {
        match value {
            Hori::Left => Self::Least,
            Hori::Right => Self::Most,
        }
    }
}

impl From<Vert> for Side {
    fn from(value: Vert) -> Self {
        match value {
            Vert::Top => Self::Least,
            Vert::Bottom => Self::Most,
        }
    }
}
