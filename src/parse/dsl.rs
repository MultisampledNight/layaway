//! Parses an unnamed DSL using [`chumsky`] into [`Layout`].
//!
//! The DSL's goal is to concisely describe layouts
//! so that it becomes less tedious to express common setups like
//! "laptop screen is centered under the external one"
//! or "DP1, DP2, DP3 are placed from left to right and
//! align their upper corners".
//!
//! With this in mind, the syntax derives rather easily.
//! Essentially, outputs, called screens, are listed after each other.
//! For each screen, only the connector to it on is required.
//! They are concatenated using `+`.
//! Hence, one example might be:
//!
//! ```text
//! vga3 + dp + edp
//! ```
//!
//! This would place the screens on
//!     VGA port 3,
//!     `DisplayPort` 1 and
//!     Embedded `DisplayPort` 1
//!         (probably a laptop internal one)
//! from left to right,
//! with their upper corners
//! touching each other.
//!
//! However, their upper corners touching
//! is just a side effect of the positioning system.
//! For each screen, one can also specify
//! its relative position
//! by listing it after the connector in question,
//! separating them using `/`.
//! For example, to place
//!     the `DisplayPort` one in the center,
//!     the embedded `DisplayPort` at the bottom and
//!     the VGA one above them all,
//! all horizontally centered, one could use:
//!
//! ```text
//! dp + edp/bottom,center + vga/top,center
//! ```
//!
//! ## Positioning
//!
//! Behind the scenes, layouting looks at the bounding box
//! of all combined screens until now
//! and then uses the specified position
//! to decide where to place it.
//! Let's call the bounding box _A_
//! and the current screen _B_.
//! The position is a bit of a headache though.
//!
//! First, it specifies the *edge*
//! that _A_ and _B_ share, as seen from _A_.
//! That has to be one of `left`, `right`, `top` or `bottom`.
//! So `right` means that _B_ is placed
//!     on the **right** side of _A_,
//!     which is also the default
//!     if the position is not specified.
//!
//! Second, it may then, after a comma, optionally specify
//! where exactly _B_ is placed on the selected edge of _A_:
//!
//! - If the shared edge was `left` or `right`:
//!     - Second part has to be one of `top`, `center` or `bottom`.
//!     - In that case, `top` is the default.
//! - If the shared edge was `top` or `bottom`:
//!     - Second part has to be one of `left`, `center` or `right`.
//!     - In that case, `center` is the default.
//!
//! Using `center` means to place _B_
//! such that the midpoints of _A_ and _B_ align.
//! Otherwise, the directions are interpreted as the corners
//! that should align.
//!
//! For example, the position `top,left` would place _B_
//!     on the **upper** edge of _A_,
//!     so that the **lower left** corner of _B_
//!     touches the upper left corner of _A_.
//! `left,top` on the other hand would place _B_
//!     on the **left** edge of _A_,
//!     so that the **upper right** corner of _B_
//!     touches the upper left corner of _A_.
//!
//! # [ABNF]
//!
//! ```ebnf
//! layout = screen *(sp "+" sp screen)
//! screen =           port
//!         [sp "@" sp resolution]
//!         [sp ":" sp scale]
//!         [sp "#" sp transform]
//!         [sp "/" sp pos]
//!
//! port = connector sp [number]
//! connector = "edp" / "hdmi" / "dp"
//!           / ? all other Connector variants in src/info.rs ?
//!
//! resolution = "720p" / "1080p" / "1200p" / "4k"
//!            / ? all other Resolution variants in src/info.rs ?
//!            ; custom resolution for more niche cases
//!            / size
//! size = number sp "x" sp number
//!
//! scale = float
//!
//! transform = ["flip"  sp] quarter-deg
//!           /  "flip" [sp  quarter-deg]
//! quarter-deg = "0" / "90" / "180" / "270"
//!
//! pos = hori [sp "," sp vert-spec]
//!     / vert [sp "," sp hori-spec]
//! hori = "left" / "right"
//! vert = "top" / "bottom"
//! hori-spec = hori / "center"
//! vert-spec = vert / "center"
//!
//! sp = *(WSP / CR / LF)
//! number = 1*DIGIT
//! float = number ["." number]
//! ```
//!
//! # Notes
//!
//! - `port` number defaults to `1`
//! - `resolution` fetches the screen resolution from the WM
//!   if left unspecified
//! - `scale` always defaults to `1` if unspecified
//!   and the screen isn't connected yet either
//! - `transform`'s rotation is clockwise
//! - `pos`
//!     - Defaults to `right,top`
//!         - If the `hori` version of pos is chosen, but no spec, `top` is assumed
//!         - If the `vert` version of pos is chosen, but no spec, `center` is assumed
//!     - Specifies on where to place the current screen
//!       referring to the entire bounding box
//!       of all layout until now
//!       so that the maximum edge is shared
//!       while the position is still fulfilled
//!
//! [ABNF]: https://datatracker.ietf.org/doc/html/rfc5234
use std::{error::Error, fmt, str::FromStr};

use chumsky::{error::Simple, prelude::*, text::whitespace, Parser};

use crate::{
    comms::Port,
    geometry::{Hori, HoriSpec, Rotation, Size, Transform, Vert, VertSpec},
    info::{Connector, Resolution},
    relative::{Layout, Position, Screen},
};

impl FromStr for Layout {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        layout().parse(s).map_err(ParseError)
    }
}

#[derive(Debug)]
pub struct ParseError(Vec<Simple<char>>);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let [err] = self.0.as_slice() {
            writeln!(f, "{err}")?;
        } else {
            writeln!(f, "{} errors encountered:", self.0.len())?;

            for (i, err) in self.0.iter().enumerate() {
                writeln!(f, "{}: {}", i + 1, err)?;
            }
        }

        write!(
            f,
            "\nfwiw this makeshift error will be replaced by ariadne... sometime"
        )
    }
}

impl Error for ParseError {}

#[must_use]
pub fn layout() -> impl Parser<char, Layout, Error = Simple<char>> {
    screen()
        .separated_by(just('+').padded())
        .then_ignore(end())
        .map(|screens| Layout { screens })
}

#[must_use]
pub fn screen() -> impl Parser<char, Screen, Error = Simple<char>> {
    port()
        .then(just('@').padded().ignore_then(resolution()).or_not())
        .then(just(':').padded().ignore_then(scale()).or_not())
        .then(just('#').padded().ignore_then(transform()).or_not())
        .then(just('/').padded().ignore_then(pos()).or_not())
        .map(|((((port, resolution), scale), transform), pos)| Screen {
            port,
            resolution,
            scale,
            transform: transform.unwrap_or_default(),
            pos: pos.unwrap_or_default(),
        })
}

#[allow(clippy::missing_panics_doc)] // cannot panic since that'd mean parsing failed already
#[must_use]
pub fn port() -> impl Parser<char, Port, Error = Simple<char>> {
    Connector::parse_from_name()
        .then(text::int(10).or_not())
        .map(|(kind, idx)| Port {
            kind,
            idx: idx.map_or(1, |idx| idx.parse().unwrap()),
        })
}

#[must_use]
pub fn resolution() -> impl Parser<char, Resolution, Error = Simple<char>> {
    choice((
        Resolution::parse_from_name(),
        size().map(Resolution::Custom),
    ))
}

#[allow(clippy::missing_panics_doc)] // cannot panic since that'd mean parsing failed already
#[must_use]
pub fn size() -> impl Parser<char, Size, Error = Simple<char>> {
    todo()
}

#[allow(clippy::missing_panics_doc)] // cannot panic since that'd mean parsing failed already
#[must_use]
pub fn scale() -> impl Parser<char, f64, Error = Simple<char>> {
    text::digits(10)
        .then(just('.').ignore_then(text::digits(10)).or_not())
        .map(|(natural, frac)| {
            if let Some(frac) = frac {
                format!("{natural}.{frac}")
            } else {
                natural
            }
            .parse()
            .unwrap()
        })
}

#[must_use]
pub fn transform() -> impl Parser<char, Transform, Error = Simple<char>> {
    let flip = just("flip").then_ignore(whitespace());

    choice((
        flip.or_not().then(rotation().map(Some)),
        flip.map(Some).then(rotation().or_not()),
    ))
    .map(|(flip, rotation)| Transform {
        flipped: flip.is_some(),
        rotation: rotation.unwrap_or_default(),
    })
}

#[must_use]
pub fn rotation() -> impl Parser<char, Rotation, Error = Simple<char>> {
    let none = just('0').to(Rotation::None);
    let quarter = just("90").to(Rotation::Quarter);
    let half = just("180").to(Rotation::Half);
    let three_quarter = just("270").to(Rotation::ThreeQuarter);

    choice((none, quarter, half, three_quarter))
}

#[must_use]
pub fn pos() -> impl Parser<char, Position, Error = Simple<char>> {
    let hori_then_vert = hori().then(just(',').padded().ignore_then(vert_spec()).or_not());
    let vert_then_hori = vert().then(just(',').padded().ignore_then(hori_spec()).or_not());

    choice((
        hori_then_vert.map(|(hori, vert)| Position::Hori {
            edge: hori,
            spec: vert.unwrap_or_default(),
        }),
        vert_then_hori.map(|(vert, hori)| Position::Vert {
            edge: vert,
            spec: hori.unwrap_or_default(),
        }),
    ))
}

pub fn separated<T, U>(
    a: impl Parser<char, T, Error = Simple<char>>,
    b: impl Parser<char, U, Error = Simple<char>>,
) -> impl Parser<char, (T, U), Error = Simple<char>> {
    a.then_ignore(just(',').padded()).then(b)
}

#[must_use]
pub fn hori() -> impl Parser<char, Hori, Error = Simple<char>> {
    let left = just("left").to(Hori::Left);
    let right = just("right").to(Hori::Right);

    choice((left, right))
}

#[must_use]
pub fn hori_spec() -> impl Parser<char, HoriSpec, Error = Simple<char>> {
    choice((hori().map(Into::into), just("center").to(HoriSpec::Center)))
}

#[must_use]
pub fn vert() -> impl Parser<char, Vert, Error = Simple<char>> {
    let top = just("top").map(|_| Vert::Top);
    let bottom = just("bottom").map(|_| Vert::Bottom);

    choice((top, bottom))
}

#[must_use]
pub fn vert_spec() -> impl Parser<char, VertSpec, Error = Simple<char>> {
    choice((vert().map(Into::into), just("center").to(VertSpec::Center)))
}
