//! Information of which the task of collection is just... tedious chores.
//!
//! Note: If there are multiple variants
//! where one of them is the prefix of the other,
//! put the longer one first.
//! Otherwise the shorter one would be always matched beforehand,
//! making the longer one unreachable.
//!
//! Note 2: All the interesting information is after the macro definitions.

use crate::geometry::Size;

use chumsky::prelude::*;
use strum::{Display, EnumString};

macro_rules! make_chumsky_parser {
    { $fn_name:ident => $( $repr:literal : $name:ident ),* $(,)? } => {
        #[must_use]
        pub fn $fn_name() -> impl Parser<char, Self, Error = Simple<char>> {
            choice([$(
                just($repr).to(Self::$name)
            ),*])
        }
    };
}

macro_rules! connectors {
    {
        $( #[$attrs:meta] )*
        ---
        $(
            $( #[$var_attrs:meta] )*
            $( $dslrepr:literal )|+
            => $wmrepr:literal
            @ $name:ident
        ),* $(,)?
    } => {
        $( #[$attrs] )*
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, EnumString)]
        pub enum Connector {$(
            #[strum(serialize = $wmrepr)]
            $name
        ),*}

        impl Connector {
            make_chumsky_parser! { parse_from_name => $($( $dslrepr : $name ),*),* }
        }
    }
}

macro_rules! resolutions {
    {
        $( #[$attrs:meta] )*
        ---
        $(
            $width:literal
            x $height:literal
            => $dslrepr:literal
            @ $name:ident
        ),* $(,)?
    } => {
        $( #[$attrs] )*
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Resolution {
            $( $name, )*
            Custom(Size),
        }

        impl Resolution {
            #[must_use] pub const fn size(&self) -> Size {
                match self {
                    $( Self::$name => Size {
                        width: $width,
                        height: $height,
                    }, )*
                    Self::Custom(size) => *size,
                }
            }

            make_chumsky_parser! { parse_from_name => $( $dslrepr : $name ),* }
        }
    };
}

connectors! {
    /// Protocol and possibly physical form of the cable/plug
    /// used to connect an output to the system.
    ///
    /// Names taken from:
    ///
    /// - <https://gitlab.freedesktop.org/mesa/drm/-/blob/362b5b0a886bdfbb92d2f78708ac7a67ee449b2d/xf86drmMode.c#L1784>
    // Ordered after how they appear in the source code listed above
    // (except for `hdmib` because `hdmia` has the alias `hdmi`, see the doccomment of this
    // module).

    ---

    "unknown" => "Unknown" @ Unknown,

    "vga" => "VGA" @ Vga,

    "dvii" => "DVI-I" @ DviI,
    "dvid" => "DVI-D" @ DviD,
    "dvia" => "DVI-A" @ DviA,

    "composite" => "Composite" @ Composite,
    "svideo" => "SVIDEO" @ Svideo,
    "lvds" => "LVDS" @ Lvds,
    "component" => "Component" @ Component,
    "din" => "DIN" @ NinePinDin,

    "dp" => "DP" @ DisplayPort,
    "hdmib" => "HDMI-B" @ HdmiB,
    "hdmia" | "hdmi" => "HDMI-A" @ HdmiA,

    "tv" => "TV" @ Tv,

    "edp" => "eDP" @ Edp,
    "virtual" => "Virtual" @ Virtual,

    "dsi" => "DSI" @ Dsi,
    "dpi" => "DPI" @ Dpi,
    "writeback" => "Writeback" @ Writeback,
    "spi" => "SPI" @ Spi,
    "usb" => "USB" @ Usb,

    // more special case -- used by Sway for `swaymsg create_output`
    "headless" => "HEADLESS" @ Headless,
}

resolutions! {
    /// Descendingly sorted by height.
    /// Taken from
    /// <https://en.wikipedia.org/wiki/Display_resolution_standards>,
    /// licensed under
    /// [CC BY-SA 4.0](https://en.wikipedia.org/wiki/Wikipedia:Text_of_the_Creative_Commons_Attribution-ShareAlike_4.0_International_License).
    /// Hence, the table below is also under CC BY-SA 4.0.
    ///
    /// Display resolutions are weird.
    /// The mapping between name and sizes is sometimes ambiguous
    /// with multiple sizes sharing the same name.
    /// This is mostly ignored here, and
    /// one of the names is picked as it seems to fit best
    /// based on how common the author perceives it to be.

    ---

     320 x  240 =>    "240p" @    Qvga,
     400 x  240 =>   "w240p" @   Wqvga,
     640 x  480 =>    "480p" @     Vga,
     800 x  480 =>   "w480p" @    Wvga,
     854 x  480 =>  "uw480p" @   Fwvga,
     960 x  540 =>    "540p" @     Qhd,
     800 x  600 =>    "600p" @    Svga,
    1024 x  600 =>   "w600p" @   Wsvga,
    1280 x  720 =>    "720p" @      Hd,
    1024 x  768 =>    "768p" @     Xga,
    1152 x  864 =>    "864p" @     XgaPlus,
    1600 x  900 =>    "900p" @      HdPlus,
    1280 x  960 =>    "960p" @ QuadVga,
    1280 x 1024 =>   "1024p" @    Sxga,
    1920 x 1080 =>   "1080p" @     Fhd,
    2048 x 1080 =>   "dci2k" @   Dci2k,
    2560 x 1080 =>  "w1080p" @   Uwfhd,
    2048 x 1152 =>   "1152p" @   Qwxga,
    1600 x 1200 =>  "s1200p" @    Uxga,
    1900 x 1200 =>   "1200p" @     FhdPlus,
    2560 x 1440 =>   "1440p" @    Wqhd,
    3440 x 1440 =>  "w1440p" @   Uwqhd,
    2256 x 1504 =>      "2k" @    Hd2k,
    2048 x 1600 =>   "1600p" @    Qxga,
    2560 x 1600 =>  "w1600p" @   Wqxga,
    3840 x 1600 => "uw1600p" @   UwqhdPlus,
    1620 x 2880 =>      "3k" @    Hd3k,
    2880 x 1800 =>   "1800p" @   WqxgaPlus,
    3200 x 1800 =>  "w1800p" @    WqhdPlus,
    1920 x 1920 => "sq1920p" @    Sqhd,
    3072 x 1920 =>   "1920p" @    Hd3kPlus,
    2560 x 2048 =>   "2048p" @   Qsxga,
    2800 x 2100 =>   "2100p" @   QsxgaPlus,
    3456 x 2160 =>    "3.5k" @    Hd3Halfk,
    3840 x 2160 =>      "4k" @   Uhd4k,
    4096 x 2160 =>   "dci4k" @   Dci4k,
    3200 x 2400 =>   "2400p" @   Quxga,
    3840 x 2400 =>  "w2400p" @   Uhd4kPlus,
    5120 x 2880 =>      "5k" @   Uhd5k,
    6144 x 3456 =>      "6k" @   Uhd6k,
    7680 x 4320 =>      "8k" @   Uhd8k,
}
