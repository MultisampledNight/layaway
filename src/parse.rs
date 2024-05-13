use std::str::FromStr;

use chumsky::{error::Simple, prelude::*, Parser};

use crate::relative::RelativeLayout;

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
//     when viewing the entire bounding box
//     of all layout until now

pub fn layout() -> impl Parser<char, RelativeLayout, Error = Simple<char>> {
    let abbrev = |a, b| shorten(just(a), just(b));

    let left = abbrev('l', "eft");
    let center = abbrev('c', "enter");
    let right = abbrev('r', "ight");

    let top = abbrev('t', "op");
    let horizon = abbrev('h', "orizon");
    let bottom = abbrev('b', "ottom");

    let hori = choice((left, center, right));
    let vert = choice((top, horizon, bottom));

    let _ = dbg!(hori.parse("left,top"));
    hori.then(vert).map(|_| todo!())
}

pub fn shorten<T, U>(
    required: impl Parser<char, T, Error = Simple<char>>,
    optional: impl Parser<char, U, Error = Simple<char>>,
) -> impl Parser<char, T, Error = Simple<char>> {
    required.then_ignore(choice((optional.ignored(), empty())))
}
