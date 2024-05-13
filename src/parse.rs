use std::str::FromStr;

use chumsky::{error::Simple, prelude::*, Parser};

use crate::{
    info::Resolution,
    relative::{Horizontal, Position, RelativeLayout, Vertical},
};

impl FromStr for RelativeLayout {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let _ = layout().parse(s);
        todo!()
    }
}

// ABNF of the unnamed DSL's syntax:
//
// layout = screen *(sp "+" sp screen)
// screen =           port
//         [sp "@" sp resolution]
//         [sp ":" sp scale]
//         [sp "/" sp pos]
// sp = *(WSP / CR / LF)
//
// port = plug-type sp [number]
// connector = "edp" / "hdmi" / "dp"
//           / ? all other Connector variants in src/info.rs ?
// number = 1*DIGIT
//
// resolution = "720p" / "1080p" / "1200p" / "4k"
//            / ? all other Resolution variants in src/info.rs ?
//
// scale = float
// float = 1*DIGIT ["." 1*DIGIT]
//
// pos =   hori
//       / vert
//       / hori sp "," sp vert
//       / vert sp "," sp hori
// hori = "left"
//      / "center"
//      / "right"
// vert = "top"
//      / "horizon"
//      / "bottom"
//
// notes:
// - connector number defaults to "1"
// - resolution defaults to "1080p"
// - scale defaults to "1" if under 4k, otherwise "2"
// - pos
//   - defaults to "right,top"
//   - specifies on where to place the current screen
//     referring to the entire bounding box
//     of all layout until now
//     so that the maximum edge is shared
//     while the position is still fulfilled

pub fn layout() -> impl Parser<char, RelativeLayout, Error = Simple<char>> {
    let _ = dbg!(pos().parse("bottom,center"));
    todo()
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
