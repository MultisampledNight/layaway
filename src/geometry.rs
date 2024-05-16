pub type Pixel = i32;

/// Rectangle in pixels.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

impl Rect {
    pub fn contains(&self, subject: Point) -> bool {
        self.x.contains(subject.x) && self.y.contains(subject.y)
    }

    /// If `target` is outside of the rect,
    /// move corners of the rect to exactly include it.
    /// Otherwise, do nothing.
    pub fn stretch_to(&mut self, target: Point) {
        self.x.stretch_to(target.x);
        self.y.stretch_to(target.y);
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
    pub fn new(a: Pixel, b: Pixel) -> Self {
        let (start, end) = if b < a { (b, a) } else { (a, b) };

        Self { start, end }
    }

    pub fn start(&self) -> Pixel {
        self.start
    }

    pub fn end(&self) -> Pixel {
        self.end
    }

    pub fn contains(&self, subject: Pixel) -> bool {
        self.start <= subject && subject <= self.end
    }

    /// If `target` is outside the interval,
    /// move the bound which is nearer to be `target` instead.
    /// Otherwise, it's inside, and do nothing.
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
}
