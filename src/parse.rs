//! Parses an unnamed DSL using [`chumsky`] into [`RelativeLayout`].
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
//! ```
//! vga3 + dp + edp
//! ```
//!
//! This would place the screens on
//!     VGA port 3,
//!     DisplayPort 1 and
//!     Embedded DisplayPort 1
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
//!     the DisplayPort one in the center,
//!     the embedded DisplayPort at the bottom and
//!     the VGA one above them all,
//! all horizontally centered, one could use:
//!
//! ```
//! dp + edp/bottom,center + vga/top,center
//! ```
//!
//! Behind the scenes, layouting looks at the bounding box
//! of all combined screens until now
//! and then uses the specified position
//! to decide where to place it
//! so that the screens and the bounding box share an edge.
//!
//! The position can specify
//!     `left`,
//!     `center`,
//!     `right`
//!     for horizontal positioning,
//! and
//!     `top`,
//!     `horizon` (just vertical center),
//!     `bottom`
//!     for vertical positioning,
//! with the first and last one of each
//! aligning the corners of the bounding box and
//! the current screen appropriately.
//!
//! # [ABNF]
//!
//! ```
//! layout = screen *(sp "+" sp screen)
//! screen =           port
//!         [sp "@" sp resolution]
//!         [sp ":" sp scale]
//!         [sp "/" sp pos]
//! sp = *(WSP / CR / LF)
//!
//! port = plug-type sp [number]
//! connector = "edp" / "hdmi" / "dp"
//!           / ? all other Connector variants in src/info.rs ?
//! number = 1*DIGIT
//!
//! resolution = "720p" / "1080p" / "1200p" / "4k"
//!            / ? all other Resolution variants in src/info.rs ?
//!
//! scale = float
//! float = 1*DIGIT ["." 1*DIGIT]
//!
//! pos =   hori
//!       / vert
//!       / hori sp "," sp vert
//!       / vert sp "," sp hori
//! hori = "left"
//!      / "center"
//!      / "right"
//! vert = "top"
//!      / "horizon"
//!      / "bottom"
//! ```
//!
//! # Notes
//!
//! - `port` number defaults to `1`
//! - `resolution` fetches the screen resolution from Sway
//!   if left unspecified
//! - `scale` defaults to `1`
//!   if `resolution`
//!   (if unspecified, the fetched one) is under 4k,
//!   otherwise `2`
//! - `pos`
//!     - Defaults to `right,top`
//!     - Specifies on where to place the current screen
//!       referring to the entire bounding box
//!       of all layout until now
//!       so that the maximum edge is shared
//!       while the position is still fulfilled
//!
//! [ABNF]: https://datatracker.ietf.org/doc/html/rfc5234
use std::str::FromStr;

use chumsky::{error::Simple, prelude::*, Parser};

use crate::{
    info::{Connector, Resolution},
    relative::{Horizontal, Position, RelativeLayout, Screen, Vertical},
    Port,
};

impl FromStr for RelativeLayout {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        layout().parse(s)
    }
}

pub fn layout() -> impl Parser<char, RelativeLayout, Error = Simple<char>> {
    screen()
        .separated_by(just('+').padded())
        .map(|screens| RelativeLayout { screens })
}

pub fn screen() -> impl Parser<char, Screen, Error = Simple<char>> {
    let resolution = Resolution::parser;
    port()
        .then(just('@').padded().ignore_then(resolution()).or_not())
        .then(just(':').padded().ignore_then(scale()).or_not())
        .then(just('/').padded().ignore_then(pos()).or_not())
        .map(|(((port, resolution), scale), pos)| Screen {
            port,
            resolution,
            scale,
            pos: pos.unwrap_or_default(),
        })
}

pub fn port() -> impl Parser<char, Port, Error = Simple<char>> {
    Connector::parser()
        .then(text::int(10).or_not())
        .map(|(kind, idx)| Port {
            kind,
            idx: idx.map(|idx| idx.parse().unwrap()).unwrap_or(1),
        })
}

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

pub fn pos() -> impl Parser<char, Position, Error = Simple<char>> {
    choice((
        separated(hori(), vert()).map(|(hori, vert)| Position { hori, vert }),
        separated(vert(), hori()).map(|(vert, hori)| Position { hori, vert }),
        hori().map(|hori| Position { hori, ..default() }),
        vert().map(|vert| Position { vert, ..default() }),
    ))
}

pub fn separated<T, U>(
    a: impl Parser<char, T, Error = Simple<char>>,
    b: impl Parser<char, U, Error = Simple<char>>,
) -> impl Parser<char, (T, U), Error = Simple<char>> {
    a.then_ignore(just(',').padded()).then(b)
}

pub fn hori() -> impl Parser<char, Horizontal, Error = Simple<char>> {
    let left = just("left").map(|_| Horizontal::Left);
    let center = just("center").map(|_| Horizontal::Center);
    let right = just("right").map(|_| Horizontal::Right);

    choice((left, center, right))
}

pub fn vert() -> impl Parser<char, Vertical, Error = Simple<char>> {
    let top = just("top").map(|_| Vertical::Top);
    let horizon = just("horizon").map(|_| Vertical::Horizon);
    let bottom = just("bottom").map(|_| Vertical::Bottom);

    choice((top, horizon, bottom))
}

fn default<T: Default>() -> T {
    T::default()
}
