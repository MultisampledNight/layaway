pub type Pixel = i32;

/// Rectangle in pixels.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect {
    pub x: Interval,
    pub y: Interval,
}

impl Rect {
    pub fn vertices(&self) -> [Point; 4] {
        [
            (self.x.start(), self.y.start()),
            (self.x.end(), self.y.start()),
            (self.x.start(), self.y.end()),
            (self.x.end(), self.y.end()),
        ]
        .map(|(x, y)| Point { x, y })
    }

    pub fn size(&self) -> Size {
        Size {
            width: self.x.len(),
            height: self.y.len(),
        }
    }

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
            self.stretch_to_point(vertex)
        }
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

    pub fn mid(&self) -> Pixel {
        (self.start + self.end) / 2
    }

    pub fn len(&self) -> Pixel {
        self.end - self.start
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
