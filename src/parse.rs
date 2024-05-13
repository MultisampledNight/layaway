use std::str::FromStr;

use chumsky::{error::Simple, primitive::todo, Parser};

use crate::relative::RelativeLayout;

impl FromStr for RelativeLayout {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        layout().parse(s)
    }
}

pub fn layout() -> impl Parser<char, RelativeLayout, Error = Simple<char>> {
    todo()
}
