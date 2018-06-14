use x11::xlib;
use xorg::XAllocColor;

pub enum NamedColor {
    AliceBlue,
    AntiqueWhite,
    AntiqueWhite1,
    AntiqueWhite2,
    AntiqueWhite3,
    AntiqueWhite4,
    Aquamarine,
    Aquamarine1,
    Aquamarine2,
    Aquamarine3,
    Aquamarine4,
    Azure,
    Azure1,
    Azure2,
    Azure3,
    Azure4,
    Beige,
    Bisque,
    Bisque1,
    Bisque2,
    Bisque3,
    Bisque4,
    Black,
    BlanchedAlmond,
    Blue,
    Blue1,
    Blue2,
    Blue3,
    Blue4,
    BlueViolet,
    Brown,
    Brown1,
    Brown2,
    Brown3,
    Brown4,
    Burlywood,
    Burlywood1,
    Burlywood2,
    Burlywood3,
    Burlywood4,
    CadetBlue,
    CadetBlue1,
    CadetBlue2,
    CadetBlue3,
    CadetBlue4,
    Chartreuse,
    Chartreuse1,
    Chartreuse2,
    Chartreuse3,
    Chartreuse4,
    Chocolate,
    Chocolate1,
    Chocolate2,
    Chocolate3,
    Chocolate4,
    Coral,
    Coral1,
    Coral2,
    Coral3,
    Coral4,
    CornflowerBlue,
    Cornsilk,
    Cornsilk1,
    Cornsilk2,
    Cornsilk3,
    Cornsilk4,
    Cyan,
    Cyan1,
    Cyan2,
    Cyan3,
    Cyan4,
    DarkBlue,
    DarkCyan,
    DarkGoldenrod,
    DarkGoldenrod1,
    DarkGoldenrod2,
    DarkGoldenrod3,
    DarkGoldenrod4,
    DarkGray,
    DarkGreen,
    DarkGrey,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOliveGreen1,
    DarkOliveGreen2,
    DarkOliveGreen3,
    DarkOliveGreen4,
    DarkOrange,
    DarkOrange1,
    DarkOrange2,
    DarkOrange3,
    DarkOrange4,
    DarkOrchid,
    DarkOrchid1,
    DarkOrchid2,
    DarkOrchid3,
    DarkOrchid4,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSeaGreen1,
    DarkSeaGreen2,
    DarkSeaGreen3,
    DarkSeaGreen4,
    DarkSlateBlue,
    DarkSlateGray,
    DarkSlateGray1,
    DarkSlateGray2,
    DarkSlateGray3,
    DarkSlateGray4,
    DarkSlateGrey,
    DarkTurquoise,
    DarkViolet,
    DebianRed,
    DeepPink,
    DeepPink1,
    DeepPink2,
    DeepPink3,
    DeepPink4,
    DeepSkyBlue,
    DeepSkyBlue1,
    DeepSkyBlue2,
    DeepSkyBlue3,
    DeepSkyBlue4,
    DimGray,
    DimGrey,
    DodgerBlue,
    DodgerBlue1,
    DodgerBlue2,
    DodgerBlue3,
    DodgerBlue4,
    Firebrick,
    Firebrick1,
    Firebrick2,
    Firebrick3,
    Firebrick4,
    FloralWhite,
    ForestGreen,
    Gainsboro,
    GhostWhite,
    Gold,
    Gold1,
    Gold2,
    Gold3,
    Gold4,
    Goldenrod,
    Goldenrod1,
    Goldenrod2,
    Goldenrod3,
    Goldenrod4,
    Gray,
    Gray0,
    Gray1,
    Gray10,
    Gray100,
    Gray11,
    Gray12,
    Gray13,
    Gray14,
    Gray15,
    Gray16,
    Gray17,
    Gray18,
    Gray19,
    Gray2,
    Gray20,
    Gray21,
    Gray22,
    Gray23,
    Gray24,
    Gray25,
    Gray26,
    Gray27,
    Gray28,
    Gray29,
    Gray3,
    Gray30,
    Gray31,
    Gray32,
    Gray33,
    Gray34,
    Gray35,
    Gray36,
    Gray37,
    Gray38,
    Gray39,
    Gray4,
    Gray40,
    Gray41,
    Gray42,
    Gray43,
    Gray44,
    Gray45,
    Gray46,
    Gray47,
    Gray48,
    Gray49,
    Gray5,
    Gray50,
    Gray51,
    Gray52,
    Gray53,
    Gray54,
    Gray55,
    Gray56,
    Gray57,
    Gray58,
    Gray59,
    Gray6,
    Gray60,
    Gray61,
    Gray62,
    Gray63,
    Gray64,
    Gray65,
    Gray66,
    Gray67,
    Gray68,
    Gray69,
    Gray7,
    Gray70,
    Gray71,
    Gray72,
    Gray73,
    Gray74,
    Gray75,
    Gray76,
    Gray77,
    Gray78,
    Gray79,
    Gray8,
    Gray80,
    Gray81,
    Gray82,
    Gray83,
    Gray84,
    Gray85,
    Gray86,
    Gray87,
    Gray88,
    Gray89,
    Gray9,
    Gray90,
    Gray91,
    Gray92,
    Gray93,
    Gray94,
    Gray95,
    Gray96,
    Gray97,
    Gray98,
    Gray99,
    Green,
    Green1,
    Green2,
    Green3,
    Green4,
    GreenYellow,
    Grey,
    Grey0,
    Grey1,
    Grey10,
    Grey100,
    Grey11,
    Grey12,
    Grey13,
    Grey14,
    Grey15,
    Grey16,
    Grey17,
    Grey18,
    Grey19,
    Grey2,
    Grey20,
    Grey21,
    Grey22,
    Grey23,
    Grey24,
    Grey25,
    Grey26,
    Grey27,
    Grey28,
    Grey29,
    Grey3,
    Grey30,
    Grey31,
    Grey32,
    Grey33,
    Grey34,
    Grey35,
    Grey36,
    Grey37,
    Grey38,
    Grey39,
    Grey4,
    Grey40,
    Grey41,
    Grey42,
    Grey43,
    Grey44,
    Grey45,
    Grey46,
    Grey47,
    Grey48,
    Grey49,
    Grey5,
    Grey50,
    Grey51,
    Grey52,
    Grey53,
    Grey54,
    Grey55,
    Grey56,
    Grey57,
    Grey58,
    Grey59,
    Grey6,
    Grey60,
    Grey61,
    Grey62,
    Grey63,
    Grey64,
    Grey65,
    Grey66,
    Grey67,
    Grey68,
    Grey69,
    Grey7,
    Grey70,
    Grey71,
    Grey72,
    Grey73,
    Grey74,
    Grey75,
    Grey76,
    Grey77,
    Grey78,
    Grey79,
    Grey8,
    Grey80,
    Grey81,
    Grey82,
    Grey83,
    Grey84,
    Grey85,
    Grey86,
    Grey87,
    Grey88,
    Grey89,
    Grey9,
    Grey90,
    Grey91,
    Grey92,
    Grey93,
    Grey94,
    Grey95,
    Grey96,
    Grey97,
    Grey98,
    Grey99,
    Honeydew,
    Honeydew1,
    Honeydew2,
    Honeydew3,
    Honeydew4,
    HotPink,
    HotPink1,
    HotPink2,
    HotPink3,
    HotPink4,
    IndianRed,
    IndianRed1,
    IndianRed2,
    IndianRed3,
    IndianRed4,
    Ivory,
    Ivory1,
    Ivory2,
    Ivory3,
    Ivory4,
    Khaki,
    Khaki1,
    Khaki2,
    Khaki3,
    Khaki4,
    Lavender,
    LavenderBlush,
    LavenderBlush1,
    LavenderBlush2,
    LavenderBlush3,
    LavenderBlush4,
    LawnGreen,
    LemonChiffon,
    LemonChiffon1,
    LemonChiffon2,
    LemonChiffon3,
    LemonChiffon4,
    LightBlue,
    LightBlue1,
    LightBlue2,
    LightBlue3,
    LightBlue4,
    LightCoral,
    LightCyan,
    LightCyan1,
    LightCyan2,
    LightCyan3,
    LightCyan4,
    LightGoldenrod,
    LightGoldenrod1,
    LightGoldenrod2,
    LightGoldenrod3,
    LightGoldenrod4,
    LightGoldenrodYellow,
    LightGray,
    LightGreen,
    LightGrey,
    LightPink,
    LightPink1,
    LightPink2,
    LightPink3,
    LightPink4,
    LightSalmon,
    LightSalmon1,
    LightSalmon2,
    LightSalmon3,
    LightSalmon4,
    LightSeaGreen,
    LightSkyBlue,
    LightSkyBlue1,
    LightSkyBlue2,
    LightSkyBlue3,
    LightSkyBlue4,
    LightSlateBlue,
    LightSlateGray,
    LightSlateGrey,
    LightSteelBlue,
    LightSteelBlue1,
    LightSteelBlue2,
    LightSteelBlue3,
    LightSteelBlue4,
    LightYellow,
    LightYellow1,
    LightYellow2,
    LightYellow3,
    LightYellow4,
    LimeGreen,
    Linen,
    Magenta,
    Magenta1,
    Magenta2,
    Magenta3,
    Magenta4,
    Maroon,
    Maroon1,
    Maroon2,
    Maroon3,
    Maroon4,
    MediumAquamarine,
    MediumBlue,
    MediumOrchid,
    MediumOrchid1,
    MediumOrchid2,
    MediumOrchid3,
    MediumOrchid4,
    MediumPurple,
    MediumPurple1,
    MediumPurple2,
    MediumPurple3,
    MediumPurple4,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    MistyRose1,
    MistyRose2,
    MistyRose3,
    MistyRose4,
    Moccasin,
    NavajoWhite,
    NavajoWhite1,
    NavajoWhite2,
    NavajoWhite3,
    NavajoWhite4,
    Navy,
    NavyBlue,
    OldLace,
    OliveDrab,
    OliveDrab1,
    OliveDrab2,
    OliveDrab3,
    OliveDrab4,
    Orange,
    Orange1,
    Orange2,
    Orange3,
    Orange4,
    OrangeRed,
    OrangeRed1,
    OrangeRed2,
    OrangeRed3,
    OrangeRed4,
    Orchid,
    Orchid1,
    Orchid2,
    Orchid3,
    Orchid4,
    PaleGoldenrod,
    PaleGreen,
    PaleGreen1,
    PaleGreen2,
    PaleGreen3,
    PaleGreen4,
    PaleTurquoise,
    PaleTurquoise1,
    PaleTurquoise2,
    PaleTurquoise3,
    PaleTurquoise4,
    PaleVioletRed,
    PaleVioletRed1,
    PaleVioletRed2,
    PaleVioletRed3,
    PaleVioletRed4,
    PapayaWhip,
    PeachPuff,
    PeachPuff1,
    PeachPuff2,
    PeachPuff3,
    PeachPuff4,
    Peru,
    Pink,
    Pink1,
    Pink2,
    Pink3,
    Pink4,
    Plum,
    Plum1,
    Plum2,
    Plum3,
    Plum4,
    PowderBlue,
    Purple,
    Purple1,
    Purple2,
    Purple3,
    Purple4,
    Red,
    Red1,
    Red2,
    Red3,
    Red4,
    RosyBrown,
    RosyBrown1,
    RosyBrown2,
    RosyBrown3,
    RosyBrown4,
    RoyalBlue,
    RoyalBlue1,
    RoyalBlue2,
    RoyalBlue3,
    RoyalBlue4,
    SaddleBrown,
    Salmon,
    Salmon1,
    Salmon2,
    Salmon3,
    Salmon4,
    SandyBrown,
    SeaGreen,
    SeaGreen1,
    SeaGreen2,
    SeaGreen3,
    SeaGreen4,
    Seashell,
    Seashell1,
    Seashell2,
    Seashell3,
    Seashell4,
    Sienna,
    Sienna1,
    Sienna2,
    Sienna3,
    Sienna4,
    SkyBlue,
    SkyBlue1,
    SkyBlue2,
    SkyBlue3,
    SkyBlue4,
    SlateBlue,
    SlateBlue1,
    SlateBlue2,
    SlateBlue3,
    SlateBlue4,
    SlateGray,
    SlateGray1,
    SlateGray2,
    SlateGray3,
    SlateGray4,
    SlateGrey,
    Snow,
    Snow1,
    Snow2,
    Snow3,
    Snow4,
    SpringGreen,
    SpringGreen1,
    SpringGreen2,
    SpringGreen3,
    SpringGreen4,
    SteelBlue,
    SteelBlue1,
    SteelBlue2,
    SteelBlue3,
    SteelBlue4,
    Tan,
    Tan1,
    Tan2,
    Tan3,
    Tan4,
    Thistle,
    Thistle1,
    Thistle2,
    Thistle3,
    Thistle4,
    Tomato,
    Tomato1,
    Tomato2,
    Tomato3,
    Tomato4,
    Turquoise,
    Turquoise1,
    Turquoise2,
    Turquoise3,
    Turquoise4,
    Violet,
    VioletRed,
    VioletRed1,
    VioletRed2,
    VioletRed3,
    VioletRed4,
    Wheat,
    Wheat1,
    Wheat2,
    Wheat3,
    Wheat4,
    White,
    WhiteSmoke,
    Yellow,
    Yellow1,
    Yellow2,
    Yellow3,
    Yellow4,
    YellowGreen,
}

impl NamedColor {
    pub fn color(&self, display: *mut xlib::Display, cm: xlib::Colormap) -> xlib::XColor {
        match &self {
            NamedColor::AliceBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 63736,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::AntiqueWhite => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 64250,
                    green: 60395,
                    blue: 55255,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::AntiqueWhite1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 61423,
                    blue: 56283,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::AntiqueWhite2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 57311,
                    blue: 52428,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::AntiqueWhite3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 49344,
                    blue: 45232,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::AntiqueWhite4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 33667,
                    blue: 30840,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Aquamarine => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32639,
                    green: 65535,
                    blue: 54484,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Aquamarine1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32639,
                    green: 65535,
                    blue: 54484,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Aquamarine2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 30326,
                    green: 61166,
                    blue: 50886,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Aquamarine3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26214,
                    green: 52685,
                    blue: 43690,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Aquamarine4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 17733,
                    green: 35723,
                    blue: 29812,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Azure => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Azure1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Azure2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57568,
                    green: 61166,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Azure3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49601,
                    green: 52685,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Azure4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 33667,
                    green: 35723,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Beige => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62965,
                    green: 62965,
                    blue: 56540,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Bisque => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 58596,
                    blue: 50372,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Bisque1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 58596,
                    blue: 50372,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Bisque2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 54741,
                    blue: 47031,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Bisque3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 47031,
                    blue: 40606,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Bisque4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 32125,
                    blue: 27499,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Black => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::BlanchedAlmond => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 60395,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Blue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Blue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Blue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Blue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Blue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::BlueViolet => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35466,
                    green: 11051,
                    blue: 58082,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Brown => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 42405,
                    green: 10794,
                    blue: 10794,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Brown1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 16448,
                    blue: 16448,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Brown2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 15163,
                    blue: 15163,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Brown3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 13107,
                    blue: 13107,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Brown4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 8995,
                    blue: 8995,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Burlywood => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57054,
                    green: 47288,
                    blue: 34695,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Burlywood1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 54227,
                    blue: 39835,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Burlywood2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 50629,
                    blue: 37265,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Burlywood3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 43690,
                    blue: 32125,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Burlywood4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 29555,
                    blue: 21845,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::CadetBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 24415,
                    green: 40606,
                    blue: 41120,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::CadetBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39064,
                    green: 62965,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::CadetBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 36494,
                    green: 58853,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::CadetBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31354,
                    green: 50629,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::CadetBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21331,
                    green: 34438,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chartreuse => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32639,
                    green: 65535,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chartreuse1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32639,
                    green: 65535,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chartreuse2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 30326,
                    green: 61166,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chartreuse3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26214,
                    green: 52685,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chartreuse4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 17733,
                    green: 35723,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chocolate => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53970,
                    green: 26985,
                    blue: 7710,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chocolate1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 32639,
                    blue: 9252,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chocolate2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 30326,
                    blue: 8481,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chocolate3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 26214,
                    blue: 7453,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Chocolate4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 17733,
                    blue: 4883,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Coral => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 32639,
                    blue: 20560,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Coral1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 29298,
                    blue: 22102,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Coral2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 27242,
                    blue: 20560,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Coral3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 23387,
                    blue: 17733,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Coral4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 15934,
                    blue: 12079,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::CornflowerBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 25700,
                    green: 38293,
                    blue: 60909,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cornsilk => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 63736,
                    blue: 56540,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cornsilk1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 63736,
                    blue: 56540,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cornsilk2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 59624,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cornsilk3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 51400,
                    blue: 45489,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cornsilk4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 34952,
                    blue: 30840,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cyan => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cyan1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cyan2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 61166,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cyan3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 52685,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Cyan4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 35723,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkCyan => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 35723,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkGoldenrod => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 47288,
                    green: 34438,
                    blue: 2827,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkGoldenrod1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 47545,
                    blue: 3855,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkGoldenrod2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 44461,
                    blue: 3598,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkGoldenrod3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 38293,
                    blue: 3084,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkGoldenrod4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 25957,
                    blue: 2056,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkGray => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 43433,
                    green: 43433,
                    blue: 43433,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 25700,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkGrey => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 43433,
                    green: 43433,
                    blue: 43433,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkKhaki => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48573,
                    green: 47031,
                    blue: 27499,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkMagenta => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 0,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOliveGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21845,
                    green: 27499,
                    blue: 12079,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOliveGreen1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 51914,
                    green: 65535,
                    blue: 28784,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOliveGreen2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48316,
                    green: 61166,
                    blue: 26728,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOliveGreen3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 41634,
                    green: 52685,
                    blue: 23130,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOliveGreen4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 28270,
                    green: 35723,
                    blue: 15677,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrange => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 35980,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrange1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 32639,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrange2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 30326,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrange3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 26214,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrange4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 17733,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrchid => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39321,
                    green: 12850,
                    blue: 52428,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrchid1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49087,
                    green: 15934,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrchid2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45746,
                    green: 14906,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrchid3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39578,
                    green: 12850,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkOrchid4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26728,
                    green: 8738,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkRed => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSalmon => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 59881,
                    green: 38550,
                    blue: 31354,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSeaGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 36751,
                    green: 48316,
                    blue: 36751,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSeaGreen1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49601,
                    green: 65535,
                    blue: 49601,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSeaGreen2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 46260,
                    green: 61166,
                    blue: 46260,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSeaGreen3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39835,
                    green: 52685,
                    blue: 39835,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSeaGreen4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26985,
                    green: 35723,
                    blue: 26985,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSlateBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 18504,
                    green: 15677,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSlateGray => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 12079,
                    green: 20303,
                    blue: 20303,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSlateGray1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 38807,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSlateGray2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 36237,
                    green: 61166,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSlateGray3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31097,
                    green: 52685,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSlateGray4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21074,
                    green: 35723,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkSlateGrey => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 12079,
                    green: 20303,
                    blue: 20303,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkTurquoise => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 52942,
                    blue: 53713,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DarkViolet => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 38036,
                    green: 0,
                    blue: 54227,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DebianRed => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 55255,
                    green: 1799,
                    blue: 20817,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepPink => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 5140,
                    blue: 37779,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepPink1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 5140,
                    blue: 37779,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepPink2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 4626,
                    blue: 35209,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepPink3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 4112,
                    blue: 30326,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepPink4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 2570,
                    blue: 20560,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepSkyBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 49087,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepSkyBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 49087,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepSkyBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 45746,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepSkyBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 39578,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DeepSkyBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 26728,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DimGray => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26985,
                    green: 26985,
                    blue: 26985,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DimGrey => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26985,
                    green: 26985,
                    blue: 26985,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DodgerBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 7710,
                    green: 37008,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DodgerBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 7710,
                    green: 37008,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DodgerBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 7196,
                    green: 34438,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DodgerBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 6168,
                    green: 29812,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::DodgerBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 4112,
                    green: 20046,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Firebrick => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45746,
                    green: 8738,
                    blue: 8738,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Firebrick1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 12336,
                    blue: 12336,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Firebrick2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 11308,
                    blue: 11308,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Firebrick3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 9766,
                    blue: 9766,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Firebrick4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 6682,
                    blue: 6682,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::FloralWhite => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 64250,
                    blue: 61680,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::ForestGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 8738,
                    green: 35723,
                    blue: 8738,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gainsboro => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 56540,
                    green: 56540,
                    blue: 56540,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::GhostWhite => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 63736,
                    green: 63736,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gold => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 55255,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gold1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 55255,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gold2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 51657,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gold3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 44461,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gold4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 30069,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Goldenrod => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 56026,
                    green: 42405,
                    blue: 8224,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Goldenrod1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 49601,
                    blue: 9509,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Goldenrod2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 46260,
                    blue: 8738,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Goldenrod3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 39835,
                    blue: 7453,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Goldenrod4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 26985,
                    blue: 5140,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48830,
                    green: 48830,
                    blue: 48830,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray0 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 771,
                    green: 771,
                    blue: 771,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray10 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 6682,
                    green: 6682,
                    blue: 6682,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray100 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray11 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 7196,
                    green: 7196,
                    blue: 7196,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray12 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 7967,
                    green: 7967,
                    blue: 7967,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray13 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 8481,
                    green: 8481,
                    blue: 8481,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray14 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 9252,
                    green: 9252,
                    blue: 9252,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray15 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 9766,
                    green: 9766,
                    blue: 9766,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray16 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 10537,
                    green: 10537,
                    blue: 10537,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray17 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 11051,
                    green: 11051,
                    blue: 11051,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray18 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 11822,
                    green: 11822,
                    blue: 11822,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray19 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 12336,
                    green: 12336,
                    blue: 12336,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 1285,
                    green: 1285,
                    blue: 1285,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray20 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 13107,
                    green: 13107,
                    blue: 13107,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray21 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 13878,
                    green: 13878,
                    blue: 13878,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray22 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 14392,
                    green: 14392,
                    blue: 14392,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray23 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 15163,
                    green: 15163,
                    blue: 15163,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray24 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 15677,
                    green: 15677,
                    blue: 15677,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray25 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 16448,
                    green: 16448,
                    blue: 16448,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray26 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 16962,
                    green: 16962,
                    blue: 16962,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray27 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 17733,
                    green: 17733,
                    blue: 17733,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray28 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 18247,
                    green: 18247,
                    blue: 18247,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray29 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 19018,
                    green: 19018,
                    blue: 19018,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 2056,
                    green: 2056,
                    blue: 2056,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray30 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 19789,
                    green: 19789,
                    blue: 19789,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray31 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 20303,
                    green: 20303,
                    blue: 20303,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray32 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21074,
                    green: 21074,
                    blue: 21074,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray33 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21588,
                    green: 21588,
                    blue: 21588,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray34 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 22359,
                    green: 22359,
                    blue: 22359,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray35 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 22873,
                    green: 22873,
                    blue: 22873,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray36 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 23644,
                    green: 23644,
                    blue: 23644,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray37 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 24158,
                    green: 24158,
                    blue: 24158,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray38 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 24929,
                    green: 24929,
                    blue: 24929,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray39 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 25443,
                    green: 25443,
                    blue: 25443,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 2570,
                    green: 2570,
                    blue: 2570,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray40 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26214,
                    green: 26214,
                    blue: 26214,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray41 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26985,
                    green: 26985,
                    blue: 26985,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray42 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 27499,
                    green: 27499,
                    blue: 27499,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray43 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 28270,
                    green: 28270,
                    blue: 28270,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray44 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 28784,
                    green: 28784,
                    blue: 28784,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray45 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 29555,
                    green: 29555,
                    blue: 29555,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray46 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 30069,
                    green: 30069,
                    blue: 30069,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray47 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 30840,
                    green: 30840,
                    blue: 30840,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray48 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31354,
                    green: 31354,
                    blue: 31354,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray49 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32125,
                    green: 32125,
                    blue: 32125,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray5 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 3341,
                    green: 3341,
                    blue: 3341,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray50 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32639,
                    green: 32639,
                    blue: 32639,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray51 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 33410,
                    green: 33410,
                    blue: 33410,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray52 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 34181,
                    green: 34181,
                    blue: 34181,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray53 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 34695,
                    green: 34695,
                    blue: 34695,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray54 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35466,
                    green: 35466,
                    blue: 35466,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray55 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35980,
                    green: 35980,
                    blue: 35980,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray56 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 36751,
                    green: 36751,
                    blue: 36751,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray57 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 37265,
                    green: 37265,
                    blue: 37265,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray58 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 38036,
                    green: 38036,
                    blue: 38036,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray59 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 38550,
                    green: 38550,
                    blue: 38550,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray6 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 3855,
                    green: 3855,
                    blue: 3855,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray60 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39321,
                    green: 39321,
                    blue: 39321,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray61 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 40092,
                    green: 40092,
                    blue: 40092,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray62 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 40606,
                    green: 40606,
                    blue: 40606,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray63 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 41377,
                    green: 41377,
                    blue: 41377,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray64 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 41891,
                    green: 41891,
                    blue: 41891,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray65 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 42662,
                    green: 42662,
                    blue: 42662,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray66 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 43176,
                    green: 43176,
                    blue: 43176,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray67 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 43947,
                    green: 43947,
                    blue: 43947,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray68 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 44461,
                    green: 44461,
                    blue: 44461,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray69 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45232,
                    green: 45232,
                    blue: 45232,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray7 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 4626,
                    green: 4626,
                    blue: 4626,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray70 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 46003,
                    green: 46003,
                    blue: 46003,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray71 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 46517,
                    green: 46517,
                    blue: 46517,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray72 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 47288,
                    green: 47288,
                    blue: 47288,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray73 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 47802,
                    green: 47802,
                    blue: 47802,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray74 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48573,
                    green: 48573,
                    blue: 48573,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray75 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49087,
                    green: 49087,
                    blue: 49087,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray76 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49858,
                    green: 49858,
                    blue: 49858,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray77 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 50372,
                    green: 50372,
                    blue: 50372,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray78 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 51143,
                    green: 51143,
                    blue: 51143,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray79 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 51657,
                    green: 51657,
                    blue: 51657,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray8 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 5140,
                    green: 5140,
                    blue: 5140,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray80 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52428,
                    green: 52428,
                    blue: 52428,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray81 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53199,
                    green: 53199,
                    blue: 53199,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray82 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53713,
                    green: 53713,
                    blue: 53713,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray83 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 54484,
                    green: 54484,
                    blue: 54484,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray84 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 54998,
                    green: 54998,
                    blue: 54998,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray85 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 55769,
                    green: 55769,
                    blue: 55769,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray86 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 56283,
                    green: 56283,
                    blue: 56283,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray87 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57054,
                    green: 57054,
                    blue: 57054,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray88 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57568,
                    green: 57568,
                    blue: 57568,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray89 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 58339,
                    green: 58339,
                    blue: 58339,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray9 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 5911,
                    green: 5911,
                    blue: 5911,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray90 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 58853,
                    green: 58853,
                    blue: 58853,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray91 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 59624,
                    green: 59624,
                    blue: 59624,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray92 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 60395,
                    green: 60395,
                    blue: 60395,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray93 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 60909,
                    green: 60909,
                    blue: 60909,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray94 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 61680,
                    blue: 61680,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray95 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62194,
                    green: 62194,
                    blue: 62194,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray96 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62965,
                    green: 62965,
                    blue: 62965,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray97 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 63479,
                    green: 63479,
                    blue: 63479,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray98 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 64250,
                    green: 64250,
                    blue: 64250,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Gray99 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 64764,
                    green: 64764,
                    blue: 64764,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Green => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 65535,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Green1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 65535,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Green2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 61166,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Green3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 52685,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Green4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 35723,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::GreenYellow => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 44461,
                    green: 65535,
                    blue: 12079,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48830,
                    green: 48830,
                    blue: 48830,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey0 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 771,
                    green: 771,
                    blue: 771,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey10 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 6682,
                    green: 6682,
                    blue: 6682,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey100 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey11 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 7196,
                    green: 7196,
                    blue: 7196,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey12 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 7967,
                    green: 7967,
                    blue: 7967,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey13 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 8481,
                    green: 8481,
                    blue: 8481,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey14 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 9252,
                    green: 9252,
                    blue: 9252,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey15 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 9766,
                    green: 9766,
                    blue: 9766,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey16 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 10537,
                    green: 10537,
                    blue: 10537,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey17 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 11051,
                    green: 11051,
                    blue: 11051,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey18 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 11822,
                    green: 11822,
                    blue: 11822,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey19 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 12336,
                    green: 12336,
                    blue: 12336,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 1285,
                    green: 1285,
                    blue: 1285,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey20 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 13107,
                    green: 13107,
                    blue: 13107,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey21 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 13878,
                    green: 13878,
                    blue: 13878,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey22 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 14392,
                    green: 14392,
                    blue: 14392,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey23 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 15163,
                    green: 15163,
                    blue: 15163,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey24 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 15677,
                    green: 15677,
                    blue: 15677,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey25 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 16448,
                    green: 16448,
                    blue: 16448,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey26 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 16962,
                    green: 16962,
                    blue: 16962,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey27 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 17733,
                    green: 17733,
                    blue: 17733,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey28 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 18247,
                    green: 18247,
                    blue: 18247,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey29 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 19018,
                    green: 19018,
                    blue: 19018,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 2056,
                    green: 2056,
                    blue: 2056,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey30 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 19789,
                    green: 19789,
                    blue: 19789,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey31 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 20303,
                    green: 20303,
                    blue: 20303,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey32 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21074,
                    green: 21074,
                    blue: 21074,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey33 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21588,
                    green: 21588,
                    blue: 21588,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey34 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 22359,
                    green: 22359,
                    blue: 22359,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey35 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 22873,
                    green: 22873,
                    blue: 22873,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey36 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 23644,
                    green: 23644,
                    blue: 23644,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey37 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 24158,
                    green: 24158,
                    blue: 24158,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey38 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 24929,
                    green: 24929,
                    blue: 24929,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey39 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 25443,
                    green: 25443,
                    blue: 25443,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 2570,
                    green: 2570,
                    blue: 2570,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey40 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26214,
                    green: 26214,
                    blue: 26214,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey41 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26985,
                    green: 26985,
                    blue: 26985,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey42 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 27499,
                    green: 27499,
                    blue: 27499,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey43 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 28270,
                    green: 28270,
                    blue: 28270,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey44 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 28784,
                    green: 28784,
                    blue: 28784,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey45 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 29555,
                    green: 29555,
                    blue: 29555,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey46 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 30069,
                    green: 30069,
                    blue: 30069,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey47 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 30840,
                    green: 30840,
                    blue: 30840,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey48 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31354,
                    green: 31354,
                    blue: 31354,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey49 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32125,
                    green: 32125,
                    blue: 32125,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey5 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 3341,
                    green: 3341,
                    blue: 3341,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey50 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32639,
                    green: 32639,
                    blue: 32639,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey51 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 33410,
                    green: 33410,
                    blue: 33410,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey52 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 34181,
                    green: 34181,
                    blue: 34181,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey53 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 34695,
                    green: 34695,
                    blue: 34695,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey54 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35466,
                    green: 35466,
                    blue: 35466,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey55 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35980,
                    green: 35980,
                    blue: 35980,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey56 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 36751,
                    green: 36751,
                    blue: 36751,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey57 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 37265,
                    green: 37265,
                    blue: 37265,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey58 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 38036,
                    green: 38036,
                    blue: 38036,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey59 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 38550,
                    green: 38550,
                    blue: 38550,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey6 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 3855,
                    green: 3855,
                    blue: 3855,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey60 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39321,
                    green: 39321,
                    blue: 39321,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey61 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 40092,
                    green: 40092,
                    blue: 40092,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey62 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 40606,
                    green: 40606,
                    blue: 40606,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey63 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 41377,
                    green: 41377,
                    blue: 41377,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey64 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 41891,
                    green: 41891,
                    blue: 41891,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey65 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 42662,
                    green: 42662,
                    blue: 42662,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey66 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 43176,
                    green: 43176,
                    blue: 43176,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey67 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 43947,
                    green: 43947,
                    blue: 43947,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey68 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 44461,
                    green: 44461,
                    blue: 44461,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey69 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45232,
                    green: 45232,
                    blue: 45232,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey7 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 4626,
                    green: 4626,
                    blue: 4626,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey70 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 46003,
                    green: 46003,
                    blue: 46003,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey71 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 46517,
                    green: 46517,
                    blue: 46517,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey72 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 47288,
                    green: 47288,
                    blue: 47288,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey73 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 47802,
                    green: 47802,
                    blue: 47802,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey74 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48573,
                    green: 48573,
                    blue: 48573,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey75 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49087,
                    green: 49087,
                    blue: 49087,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey76 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49858,
                    green: 49858,
                    blue: 49858,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey77 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 50372,
                    green: 50372,
                    blue: 50372,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey78 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 51143,
                    green: 51143,
                    blue: 51143,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey79 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 51657,
                    green: 51657,
                    blue: 51657,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey8 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 5140,
                    green: 5140,
                    blue: 5140,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey80 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52428,
                    green: 52428,
                    blue: 52428,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey81 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53199,
                    green: 53199,
                    blue: 53199,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey82 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53713,
                    green: 53713,
                    blue: 53713,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey83 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 54484,
                    green: 54484,
                    blue: 54484,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey84 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 54998,
                    green: 54998,
                    blue: 54998,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey85 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 55769,
                    green: 55769,
                    blue: 55769,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey86 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 56283,
                    green: 56283,
                    blue: 56283,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey87 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57054,
                    green: 57054,
                    blue: 57054,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey88 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57568,
                    green: 57568,
                    blue: 57568,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey89 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 58339,
                    green: 58339,
                    blue: 58339,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey9 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 5911,
                    green: 5911,
                    blue: 5911,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey90 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 58853,
                    green: 58853,
                    blue: 58853,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey91 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 59624,
                    green: 59624,
                    blue: 59624,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey92 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 60395,
                    green: 60395,
                    blue: 60395,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey93 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 60909,
                    green: 60909,
                    blue: 60909,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey94 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 61680,
                    blue: 61680,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey95 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62194,
                    green: 62194,
                    blue: 62194,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey96 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62965,
                    green: 62965,
                    blue: 62965,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey97 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 63479,
                    green: 63479,
                    blue: 63479,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey98 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 64250,
                    green: 64250,
                    blue: 64250,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Grey99 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 64764,
                    green: 64764,
                    blue: 64764,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Honeydew => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 65535,
                    blue: 61680,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Honeydew1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 65535,
                    blue: 61680,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Honeydew2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57568,
                    green: 61166,
                    blue: 57568,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Honeydew3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49601,
                    green: 52685,
                    blue: 49601,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Honeydew4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 33667,
                    green: 35723,
                    blue: 33667,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::HotPink => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 26985,
                    blue: 46260,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::HotPink1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 28270,
                    blue: 46260,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::HotPink2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 27242,
                    blue: 42919,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::HotPink3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 24672,
                    blue: 37008,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::HotPink4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 14906,
                    blue: 25186,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::IndianRed => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 23644,
                    blue: 23644,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::IndianRed1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 27242,
                    blue: 27242,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::IndianRed2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 25443,
                    blue: 25443,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::IndianRed3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 21845,
                    blue: 21845,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::IndianRed4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 14906,
                    blue: 14906,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Ivory => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 61680,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Ivory1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 61680,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Ivory2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 61166,
                    blue: 57568,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Ivory3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 52685,
                    blue: 49601,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Ivory4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 35723,
                    blue: 33667,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Khaki => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 59110,
                    blue: 35980,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Khaki1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 63222,
                    blue: 36751,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Khaki2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 59110,
                    blue: 34181,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Khaki3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 50886,
                    blue: 29555,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Khaki4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 34438,
                    blue: 20046,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Lavender => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 59110,
                    green: 59110,
                    blue: 64250,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LavenderBlush => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 61680,
                    blue: 62965,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LavenderBlush1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 61680,
                    blue: 62965,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LavenderBlush2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 57568,
                    blue: 58853,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LavenderBlush3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 49601,
                    blue: 50629,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LavenderBlush4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 33667,
                    blue: 34438,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LawnGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31868,
                    green: 64764,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LemonChiffon => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 64250,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LemonChiffon1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 64250,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LemonChiffon2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 59881,
                    blue: 49087,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LemonChiffon3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 51657,
                    blue: 42405,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LemonChiffon4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 35209,
                    blue: 28784,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 44461,
                    green: 55512,
                    blue: 59110,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49087,
                    green: 61423,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45746,
                    green: 57311,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39578,
                    green: 49344,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26728,
                    green: 33667,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightCoral => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61680,
                    green: 32896,
                    blue: 32896,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightCyan => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57568,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightCyan1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57568,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightCyan2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53713,
                    green: 61166,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightCyan3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 46260,
                    green: 52685,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightCyan4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31354,
                    green: 35723,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGoldenrod => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 56797,
                    blue: 33410,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGoldenrod1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 60652,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGoldenrod2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 56540,
                    blue: 33410,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGoldenrod3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 48830,
                    blue: 28784,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGoldenrod4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 33153,
                    blue: 19532,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGoldenrodYellow => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 64250,
                    green: 64250,
                    blue: 53970,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGray => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 54227,
                    green: 54227,
                    blue: 54227,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 37008,
                    green: 61166,
                    blue: 37008,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightGrey => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 54227,
                    green: 54227,
                    blue: 54227,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightPink => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 46774,
                    blue: 49601,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightPink1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 44718,
                    blue: 47545,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightPink2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 41634,
                    blue: 44461,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightPink3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 35980,
                    blue: 38293,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightPink4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 24415,
                    blue: 25957,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSalmon => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 41120,
                    blue: 31354,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSalmon1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 41120,
                    blue: 31354,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSalmon2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 38293,
                    blue: 29298,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSalmon3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 33153,
                    blue: 25186,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSalmon4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 22359,
                    blue: 16962,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSeaGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 8224,
                    green: 45746,
                    blue: 43690,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSkyBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 34695,
                    green: 52942,
                    blue: 64250,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSkyBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45232,
                    green: 58082,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSkyBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 42148,
                    green: 54227,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSkyBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 36237,
                    green: 46774,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSkyBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 24672,
                    green: 31611,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSlateBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 33924,
                    green: 28784,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSlateGray => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 30583,
                    green: 34952,
                    blue: 39321,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSlateGrey => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 30583,
                    green: 34952,
                    blue: 39321,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSteelBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45232,
                    green: 50372,
                    blue: 57054,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSteelBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 51914,
                    green: 57825,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSteelBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48316,
                    green: 53970,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSteelBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 41634,
                    green: 46517,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightSteelBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 28270,
                    green: 31611,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightYellow => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 57568,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightYellow1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 57568,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightYellow2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 61166,
                    blue: 53713,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightYellow3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 52685,
                    blue: 46260,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LightYellow4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 35723,
                    blue: 31354,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::LimeGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 12850,
                    green: 52685,
                    blue: 12850,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Linen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 64250,
                    green: 61680,
                    blue: 59110,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Magenta => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 0,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Magenta1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 0,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Magenta2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 0,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Magenta3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 0,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Magenta4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 0,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Maroon => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45232,
                    green: 12336,
                    blue: 24672,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Maroon1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 13364,
                    blue: 46003,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Maroon2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 12336,
                    blue: 42919,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Maroon3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 10537,
                    blue: 37008,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Maroon4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 7196,
                    blue: 25186,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumAquamarine => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26214,
                    green: 52685,
                    blue: 43690,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumOrchid => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 47802,
                    green: 21845,
                    blue: 54227,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumOrchid1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 57568,
                    green: 26214,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumOrchid2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53713,
                    green: 24415,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumOrchid3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 46260,
                    green: 21074,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumOrchid4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31354,
                    green: 14135,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumPurple => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 37779,
                    green: 28784,
                    blue: 56283,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumPurple1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 43947,
                    green: 33410,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumPurple2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 40863,
                    green: 31097,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumPurple3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35209,
                    green: 26728,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumPurple4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 23901,
                    green: 18247,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumSeaGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 15420,
                    green: 46003,
                    blue: 29041,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumSlateBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31611,
                    green: 26728,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumSpringGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 64250,
                    blue: 39578,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumTurquoise => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 18504,
                    green: 53713,
                    blue: 52428,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MediumVioletRed => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 51143,
                    green: 5397,
                    blue: 34181,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MidnightBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 6425,
                    green: 6425,
                    blue: 28784,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MintCream => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62965,
                    green: 65535,
                    blue: 64250,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MistyRose => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 58596,
                    blue: 57825,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MistyRose1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 58596,
                    blue: 57825,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MistyRose2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 54741,
                    blue: 53970,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MistyRose3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 47031,
                    blue: 46517,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::MistyRose4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 32125,
                    blue: 31611,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Moccasin => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 58596,
                    blue: 46517,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::NavajoWhite => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 57054,
                    blue: 44461,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::NavajoWhite1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 57054,
                    blue: 44461,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::NavajoWhite2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 53199,
                    blue: 41377,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::NavajoWhite3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 46003,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::NavajoWhite4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 31097,
                    blue: 24158,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Navy => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 32896,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::NavyBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 0,
                    blue: 32896,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OldLace => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65021,
                    green: 62965,
                    blue: 59110,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OliveDrab => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 27499,
                    green: 36494,
                    blue: 8995,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OliveDrab1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 49344,
                    green: 65535,
                    blue: 15934,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OliveDrab2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 46003,
                    green: 61166,
                    blue: 14906,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OliveDrab3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39578,
                    green: 52685,
                    blue: 12850,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OliveDrab4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26985,
                    green: 35723,
                    blue: 8738,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orange => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 42405,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orange1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 42405,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orange2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 39578,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orange3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 34181,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orange4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 23130,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OrangeRed => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 17733,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OrangeRed1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 17733,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OrangeRed2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 16448,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OrangeRed3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 14135,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::OrangeRed4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 9509,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orchid => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 56026,
                    green: 28784,
                    blue: 54998,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orchid1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 33667,
                    blue: 64250,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orchid2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 31354,
                    blue: 59881,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orchid3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 26985,
                    blue: 51657,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Orchid4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 18247,
                    blue: 35209,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleGoldenrod => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 59624,
                    blue: 43690,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39064,
                    green: 64507,
                    blue: 39064,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleGreen1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39578,
                    green: 65535,
                    blue: 39578,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleGreen2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 37008,
                    green: 61166,
                    blue: 37008,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleGreen3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31868,
                    green: 52685,
                    blue: 31868,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleGreen4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21588,
                    green: 35723,
                    blue: 21588,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleTurquoise => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 44975,
                    green: 61166,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleTurquoise1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48059,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleTurquoise2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 44718,
                    green: 61166,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleTurquoise3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 38550,
                    green: 52685,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleTurquoise4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26214,
                    green: 35723,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleVioletRed => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 56283,
                    green: 28784,
                    blue: 37779,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleVioletRed1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 33410,
                    blue: 43947,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleVioletRed2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 31097,
                    blue: 40863,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleVioletRed3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 26728,
                    blue: 35209,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PaleVioletRed4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 18247,
                    blue: 23901,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PapayaWhip => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 61423,
                    blue: 54741,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PeachPuff => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 56026,
                    blue: 47545,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PeachPuff1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 56026,
                    blue: 47545,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PeachPuff2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 52171,
                    blue: 44461,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PeachPuff3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 44975,
                    blue: 38293,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PeachPuff4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 30583,
                    blue: 25957,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Peru => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 34181,
                    blue: 16191,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Pink => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 49344,
                    blue: 52171,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Pink1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 46517,
                    blue: 50629,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Pink2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 43433,
                    blue: 47288,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Pink3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 37265,
                    blue: 40606,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Pink4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 25443,
                    blue: 27756,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Plum => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 56797,
                    green: 41120,
                    blue: 56797,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Plum1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 48059,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Plum2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 44718,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Plum3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 38550,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Plum4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 26214,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::PowderBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 45232,
                    green: 57568,
                    blue: 59110,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Purple => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 41120,
                    green: 8224,
                    blue: 61680,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Purple1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39835,
                    green: 12336,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Purple2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 37265,
                    green: 11308,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Purple3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32125,
                    green: 9766,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Purple4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21845,
                    green: 6682,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Red => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Red1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Red2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Red3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Red4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 0,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RosyBrown => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 48316,
                    green: 36751,
                    blue: 36751,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RosyBrown1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 49601,
                    blue: 49601,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RosyBrown2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 46260,
                    blue: 46260,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RosyBrown3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 39835,
                    blue: 39835,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RosyBrown4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 26985,
                    blue: 26985,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RoyalBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 16705,
                    green: 26985,
                    blue: 57825,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RoyalBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 18504,
                    green: 30326,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RoyalBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 17219,
                    green: 28270,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RoyalBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 14906,
                    green: 24415,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::RoyalBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 10023,
                    green: 16448,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SaddleBrown => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 17733,
                    blue: 4883,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Salmon => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 64250,
                    green: 32896,
                    blue: 29298,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Salmon1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 35980,
                    blue: 26985,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Salmon2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 33410,
                    blue: 25186,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Salmon3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 28784,
                    blue: 21588,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Salmon4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 19532,
                    blue: 14649,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SandyBrown => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62708,
                    green: 42148,
                    blue: 24672,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SeaGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 11822,
                    green: 35723,
                    blue: 22359,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SeaGreen1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 21588,
                    green: 65535,
                    blue: 40863,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SeaGreen2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 20046,
                    green: 61166,
                    blue: 38036,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SeaGreen3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 17219,
                    green: 52685,
                    blue: 32896,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SeaGreen4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 11822,
                    green: 35723,
                    blue: 22359,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Seashell => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 62965,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Seashell1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 62965,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Seashell2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 58853,
                    blue: 57054,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Seashell3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 50629,
                    blue: 49087,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Seashell4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 34438,
                    blue: 33410,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Sienna => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 41120,
                    green: 21074,
                    blue: 11565,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Sienna1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 33410,
                    blue: 18247,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Sienna2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 31097,
                    blue: 16962,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Sienna3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 26728,
                    blue: 14649,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Sienna4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 18247,
                    blue: 9766,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SkyBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 34695,
                    green: 52942,
                    blue: 60395,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SkyBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 34695,
                    green: 52942,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SkyBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 32382,
                    green: 49344,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SkyBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 27756,
                    green: 42662,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SkyBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 19018,
                    green: 28784,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 27242,
                    green: 23130,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 33667,
                    green: 28527,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 31354,
                    green: 26471,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 26985,
                    green: 22873,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 18247,
                    green: 15420,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateGray => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 28784,
                    green: 32896,
                    blue: 37008,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateGray1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 50886,
                    green: 58082,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateGray2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 47545,
                    green: 54227,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateGray3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 40863,
                    green: 46774,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateGray4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 27756,
                    green: 31611,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SlateGrey => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 28784,
                    green: 32896,
                    blue: 37008,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Snow => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 64250,
                    blue: 64250,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Snow1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 64250,
                    blue: 64250,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Snow2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 59881,
                    blue: 59881,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Snow3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 51657,
                    blue: 51657,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Snow4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 35209,
                    blue: 35209,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SpringGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 65535,
                    blue: 32639,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SpringGreen1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 65535,
                    blue: 32639,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SpringGreen2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 61166,
                    blue: 30326,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SpringGreen3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 52685,
                    blue: 26214,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SpringGreen4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 35723,
                    blue: 17733,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SteelBlue => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 17990,
                    green: 33410,
                    blue: 46260,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SteelBlue1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 25443,
                    green: 47288,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SteelBlue2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 23644,
                    green: 44204,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SteelBlue3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 20303,
                    green: 38036,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::SteelBlue4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 13878,
                    green: 25700,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tan => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53970,
                    green: 46260,
                    blue: 35980,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tan1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 42405,
                    blue: 20303,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tan2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 39578,
                    blue: 18761,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tan3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 34181,
                    blue: 16191,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tan4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 23130,
                    blue: 11051,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Thistle => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 55512,
                    green: 49087,
                    blue: 55512,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Thistle1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 57825,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Thistle2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 53970,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Thistle3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 46517,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Thistle4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 31611,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tomato => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 25443,
                    blue: 18247,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tomato1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 25443,
                    blue: 18247,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tomato2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 23644,
                    blue: 16962,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tomato3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 20303,
                    blue: 14649,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Tomato4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 13878,
                    blue: 9766,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Turquoise => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 16448,
                    green: 57568,
                    blue: 53456,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Turquoise1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 62965,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Turquoise2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 58853,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Turquoise3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 50629,
                    blue: 52685,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Turquoise4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 0,
                    green: 34438,
                    blue: 35723,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Violet => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 33410,
                    blue: 61166,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::VioletRed => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 53456,
                    green: 8224,
                    blue: 37008,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::VioletRed1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 15934,
                    blue: 38550,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::VioletRed2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 14906,
                    blue: 35980,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::VioletRed3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 12850,
                    blue: 30840,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::VioletRed4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 8738,
                    blue: 21074,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Wheat => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62965,
                    green: 57054,
                    blue: 46003,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Wheat1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 59367,
                    blue: 47802,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Wheat2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 55512,
                    blue: 44718,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Wheat3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 47802,
                    blue: 38550,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Wheat4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 32382,
                    blue: 26214,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::White => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 65535,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::WhiteSmoke => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 62965,
                    green: 62965,
                    blue: 62965,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Yellow => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Yellow1 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 65535,
                    green: 65535,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Yellow2 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 61166,
                    green: 61166,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Yellow3 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 52685,
                    green: 52685,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::Yellow4 => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 35723,
                    green: 35723,
                    blue: 0,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
            NamedColor::YellowGreen => {
                let mut color = xlib::XColor {
                    pixel: 0,
                    red: 39578,
                    green: 52685,
                    blue: 12850,
                    flags: 0,
                    pad: 0,
                };

                XAllocColor(display, cm, &mut color);

                color
            }
        }
    }
}
