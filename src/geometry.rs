/// Math for working with rectangles and intervals.
///
/// At all places, a x+ right, y+ down coordinate system is assumed.
/// Well, except for [`Interval`] and [`Pixel`], which work in 1D.
use std::fmt;

pub type Pixel = i32;

/// Rectangle in pixels.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

impl Rect {
    #[must_use]
    pub fn vertices(&self) -> [Point; 4] {
        [
            (self.x.start(), self.y.start()),
            (self.x.end(), self.y.start()),
            (self.x.start(), self.y.end()),
            (self.x.end(), self.y.end()),
        ]
        .map(|(x, y)| Point { x, y })
    }

    #[must_use]
    pub fn size(&self) -> Size {
        Size {
            width: self.x.len(),
            height: self.y.len(),
        }
    }

    #[must_use]
    pub fn contains(&self, subject: Point) -> bool {
        self.x.contains(subject.x) && self.y.contains(subject.y)
    }

    /// If `target` is outside of the rect,
    /// move corners of the rect to exactly include it.
    /// Otherwise, do nothing.
    pub fn stretch_to_point(&mut self, target: Point) {
        self.x.stretch_to(target.x);
        self.y.stretch_to(target.y);
    }

    pub fn stretch_to_rect(&mut self, target: Self) {
        for vertex in target.vertices() {
            self.stretch_to_point(vertex);
        }
    }

    /// Divides the size by the given `factor`,
    /// such that all corners _except_ the given one move
    /// (if `factor != 1.0`).
    /// The given corner is not moved.
    ///
    /// Assumes a coordinate system where
    /// x+ is right-hand and y+ is towards bottom.
    pub fn divide_at(&mut self, corner: Corner, divisor: f64) {
        self.x.divide_at(corner.hori.into(), divisor);
        self.y.divide_at(corner.vert.into(), divisor);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: Pixel,
    pub y: Pixel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    pub width: Pixel,
    pub height: Pixel,
}

/// Range thought in pixels.
/// [`std::ops::RangeInclusive`] but not since it's too restricted
/// and does not implement `PartialOrd`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval {
    start: Pixel,
    end: Pixel,
}

impl Interval {
    /// Creates a new [`Interval`] between `a` and `b`.
    /// `b` may be less than `a`.
    #[must_use]
    pub fn new(a: Pixel, b: Pixel) -> Self {
        let (start, end) = if b < a { (b, a) } else { (a, b) };

        Self { start, end }
    }

    #[must_use]
    pub fn start(&self) -> Pixel {
        self.start
    }

    #[must_use]
    pub fn end(&self) -> Pixel {
        self.end
    }

    #[must_use]
    pub fn mid(&self) -> Pixel {
        (self.start + self.end) / 2
    }

    #[must_use]
    pub fn len(&self) -> Pixel {
        self.end - self.start
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    #[must_use]
    pub fn contains(&self, subject: Pixel) -> bool {
        self.start <= subject && subject <= self.end
    }

    /// If `target` is outside the interval,
    /// move the bound which is nearer to be `target` instead.
    /// Otherwise, it's inside, and do nothing.
    ///
    /// # Panics
    ///
    /// Panics if internal invariants are not upheld.
    /// If that happens, that's a bug.
    pub fn stretch_to(&mut self, target: Pixel) {
        if self.contains(target) {
            return;
        }

        let Self {
            ref mut start,
            ref mut end,
        } = self;

        // on which side is `target`, before `start` or after `end`?
        match (target < *start, *end < target) {
            (true, false) => *start = target,
            (false, true) => *end = target,
            _ => panic!("end is before start, meaning broken invariants"),
        }
    }

    /// Divides the length by the given `factor`
    /// such that the limit on `side`
    /// stays at the same position.
    #[allow(clippy::cast_possible_truncation)]
    pub fn divide_at(&mut self, side: Side, divisor: f64) {
        match side {
            Side::Least => self.end = self.start + (self.len() as f64 / divisor) as i32,
            Side::Most => self.start = self.end - (self.len() as f64 / divisor) as i32,
        }
    }

    /// Creates a new [`Interval`] of the given `length` next to this interval,
    /// on the given `side`.
    /// The new interval will touch this one and share one limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use layaway::geometry::{Side, Interval};
    /// let space = Interval::new(100, 200);
    /// let length = 20;
    /// assert_eq!(
    ///     space.place_outside(10, Side::Least),
    ///     Interval::new(90, 100),
    /// );
    /// ```
    #[must_use]
    pub fn place_outside(self, length: Pixel, side: Side) -> Self {
        match side {
            Side::Least => Self::new(self.start - length, self.start()),
            Side::Most => Self::new(self.end, self.end + length),
        }
    }

    /// Creates a new [`Interval`] of the given `length` inside of interval,
    /// on the given `side`.
    #[must_use]
    pub fn place_inside(self, length: Pixel, pos: MaybeCenter<Side>) -> Self {
        match pos {
            MaybeCenter::Extreme(Side::Least) => Self::new(self.start(), self.start() + length),
            MaybeCenter::Center => Self::new(self.mid() - length / 2, self.mid() + length / 2),
            MaybeCenter::Extreme(Side::Most) => Self::new(self.end - length, self.end),
        }
    }
}

/// One corner of a [`Rect`].
#[derive(Clone, Copy, Debug)]
pub struct Corner {
    /// Whether the corner is left or right.
    pub hori: Hori,
    /// Whether the corner is at the top or bottom.
    pub vert: Vert,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hori {
    Left,
    #[default]
    Right,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Vert {
    #[default]
    Top,
    Bottom,
}

pub type HoriSpec = MaybeCenter<Hori>;
pub type VertSpec = MaybeCenter<Vert>;

impl Default for HoriSpec {
    fn default() -> Self {
        Self::Center
    }
}

impl Default for VertSpec {
    fn default() -> Self {
        Self::Extreme(Vert::Top)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MaybeCenter<T: Clone + Copy + fmt::Debug> {
    Extreme(T),
    Center,
}

impl<T: Clone + Copy + fmt::Debug> MaybeCenter<T> {
    pub fn map<U: Clone + Copy + fmt::Debug>(self, op: impl FnOnce(T) -> U) -> MaybeCenter<U> {
        match self {
            Self::Center => MaybeCenter::Center,
            Self::Extreme(extreme) => MaybeCenter::Extreme(op(extreme)),
        }
    }
}

impl<T: Clone + Copy + fmt::Debug> From<T> for MaybeCenter<T> {
    fn from(value: T) -> Self {
        Self::Extreme(value)
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
