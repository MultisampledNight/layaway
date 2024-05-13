use std::str::FromStr;

use chumsky::{error::Simple, prelude::*, text::whitespace, Parser};

use crate::relative::{Horizontal, Position, RelativeLayout, Vertical};

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
// scale = 1*DIGIT ["." 1*DIGIT]
//
// pos =   hori
//       / vert
//       / hori sp "," sp vert
//       / vert sp "," sp hori
// hori = "l" ["eft"]
//      / "c" ["enter"]
//      / "r" ["ight"]
// vert = "t" ["op"]
//      / "h" ["orizon"]
//      / "b" ["ottom"]
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
    let pos = choice((
        hori().map(|hori| Position { hori, ..default() }),
        vert().map(|vert| Position { vert, ..default() }),
        separated(hori(), vert()).map(|(hori, vert)| Position { hori, vert }),
        separated(vert(), hori()).map(|(vert, hori)| Position { hori, vert }),
    ));

    let _ = dbg!(pos.parse("bottom"));
    pos.map(|_| todo!())
}

pub fn separated<T, U>(
    a: impl Parser<char, T, Error = Simple<char>>,
    b: impl Parser<char, U, Error = Simple<char>>,
) -> impl Parser<char, (T, U), Error = Simple<char>> {
    a.then_ignore(whitespace())
        .then_ignore(just(','))
        .then_ignore(whitespace())
        .then(b)
}

pub fn hori() -> impl Parser<char, Horizontal, Error = Simple<char>> {
    let left = shorten('l', "eft").map(|_| Horizontal::Left);
    let center = shorten('c', "enter").map(|_| Horizontal::Center);
    let right = shorten('r', "ight").map(|_| Horizontal::Right);

    choice((left, center, right))
}

pub fn vert() -> impl Parser<char, Vertical, Error = Simple<char>> {
    let top = shorten('t', "op").map(|_| Vertical::Top);
    let horizon = shorten('h', "orizon").map(|_| Vertical::Horizon);
    let bottom = shorten('b', "ottom").map(|_| Vertical::Bottom);

    choice((top, horizon, bottom))
}

pub fn shorten(
    required: char,
    optional: &str,
) -> impl Parser<char, char, Error = Simple<char>> + '_ {
    just(required).then_ignore(choice((just(optional).ignored(), empty())))
}

fn default<T: Default>() -> T {
    T::default()
}
