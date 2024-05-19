//! Concretizes [`relative::Layout`] into [`absolute::Layout`]

use crate::{
    absolute,
    comms::{self, Comms},
    geometry::{Interval, Pixel, Rect},
    relative::{self, Hori, HoriSpec, Position, Vert, VertSpec},
    Map,
};

impl relative::Layout {
    /// Resolve the layout according to the currently connected displays.
    pub fn to_absolute(&self, comms: &mut dyn Comms) -> comms::Result<absolute::Layout> {
        let current_absolute = comms.layout();
        let mut bounding_box = Rect {
            x: Interval::new(0, 0),
            y: Interval::new(0, 0),
        };
        //let placed = Map::new();

        for screen in &self.screens {
            match screen.pos {
                Position::Hori { edge, spec } => {
                    todo!()
                }
                Position::Vert { edge, spec } => {
                    todo!()
                }
            }
        }

        todo!()
    }
}

impl Interval {
    /// Creates a new [`Interval`] of the given `length` next to this interval,
    /// on the given `side`.
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
    pub fn place_adjacent(self, length: Pixel, side: Extreme) -> Self {
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
