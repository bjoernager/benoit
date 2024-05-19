/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit.

	benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU Af-
	fero General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU Af-
	fero General Public License along with benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::colour::Colour;

impl Colour {
	/// The web colour <span style="color: aliceblue;">`aliceblue`</span>.
	pub const ALICE_BLUE: Self = Self {
		red:   0.941176471,
		green: 0.972549020,
		blue:  1.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: antiquewhite;">`antiquewhite`</span>.
	pub const ANTIQUE_WHITE: Self = Self {
		red:   0.980392157,
		green: 0.921568627,
		blue:  0.843137255,
		alpha: 1.0,
	};

	/// The web colour <span style="color: aqua;">`aqua`</span>.
	pub const AQUA: Self = Self {
		 red:   0.0,
		 green: 1.0,
		 blue:  1.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: aquamarine;">`aquamarine`</span>.
	pub const AQUAMARINE: Self = Self {
		red:   0.498039216,
		green: 1.0,
		blue:  0.831372549,
		alpha: 1.0,
	};

	/// The web colour <span style="color: azure;">`azure`</span>.
	pub const AZURE: Self = Self {
		red:   0.941176471,
		green: 1.0,
		blue:  1.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: beige;">`beige`</span>.
	pub const BEIGE: Self = Self {
		red:   0.960784314,
		green: 0.960784314,
		blue:  0.862745098,
		alpha: 1.0,
	};

	/// The web colour <span style="color: bisque;">`bisque`</span>.
	pub const BISQUE: Self = Self {
		red:   1.0,
		green: 0.894117647,
		blue:  0.768627451,
		alpha: 1.0,
	};

	/// The web colour <span style="color: black;">`black`</span>.
	pub const BLACK: Self = Self {
		 red:   0.0,
		 green: 0.0,
		 blue:  0.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: blanchedalmond;">`blanchedalmond`</span>.
	pub const BLANCHED_ALMOND: Self = Self {
		red:   1.0,
		green: 0.921568627,
		blue:  0.803921569,
		alpha: 1.0,
	};

	/// The web colour <span style="color: blue;">`blue`</span>.
	pub const BLUE: Self = Self {
		 red:   0.0,
		 green: 0.0,
		 blue:  1.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: blueviolet;">`blueviolet`</span>.
	pub const BLUE_VIOLET: Self = Self {
		red:   0.541176471,
		green: 0.168627451,
		blue:  0.886274510,
		alpha: 1.0,
	};

	/// The web colour <span style="color: brown;">`brown`</span>.
	pub const BROWN: Self = Self {
		red:   0.647058824,
		green: 0.164705882,
		blue:  0.164705882,
		alpha: 1.0,
	};

	/// The web colour <span style="color: burlywood;">`burlywood`</span>.
	pub const BURLYWOOD: Self = Self {
		red:   0.870588235,
		green: 0.721568627,
		blue:  0.529411765,
		alpha: 1.0,
	};

	/// The web colour <span style="color: cadetblue;">`cadetblue`</span>.
	pub const CADET_BLUE: Self = Self {
		 red:   0.372549020,
		 green: 0.619607843,
		 blue:  0.627450980,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: chartreuse;">`chartreuse`</span>.
	pub const CHARTREUSE: Self = Self {
		red:   0.498039216,
		green: 1.0,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: chocolate;">`chocolate`</span>.
	pub const CHOCOLATE: Self = Self {
		red:   0.823529412,
		green: 0.411764706,
		blue:  0.117647059,
		alpha: 1.0,
	};

	/// The web colour <span style="color: coral;">`coral`</span>.
	pub const CORAL: Self = Self {
		red:   1.0,
		green: 0.498039216,
		blue:  0.313725490,
		alpha: 1.0,
	};

	/// The web colour <span style="color: cornflowerblue;">`cornflowerblue`</span>.
	pub const CORNFLOWER_BLUE: Self = Self {
		red:   0.392156863,
		green: 0.584313725,
		blue:  0.929411765,
		alpha: 1.0,
	};

	/// The web colour <span style="color: cornsilk;">`cornsilk`</span>.
	pub const CORNSILK: Self = Self {
		red:   1.0,
		green: 0.972549020,
		blue:  0.862745098,
		alpha: 1.0,
	};

	/// The web colour <span style="color: crimson;">`crimson`</span>.
	pub const CRIMSON: Self = Self {
		red:   0.862745098,
		green: 0.078431373,
		blue:  0.235294118,
		alpha: 1.0,
	};

	/// The web colour <span style="color: cyan;">`cyan`</span>.
	pub const CYAN: Self = Self {
		 red:   0.0,
		 green: 1.0,
		 blue:  1.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: darkblue;">`darkblue`</span>.
	pub const DARK_BLUE: Self = Self {
		 red:   0.0,
		 green: 0.0,
		 blue:  0.545098039,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: darkcyan;">`darkcyan`</span>.
	pub const DARK_CYAN: Self = Self {
		 red:   0.0,
		 green: 0.545098039,
		 blue:  0.545098039,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: darkgoldenrod;">`darkgoldenrod`</span>.
	pub const DARK_GOLDENROD: Self = Self {
		red:   0.721568627,
		green: 0.525490196,
		blue:  0.043137255,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darkgreen;">`darkgreen`</span>.
	pub const DARK_GREEN: Self = Self {
		 red:   0.0,
		 green: 0.392156863,
		 blue:  0.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: darkgrey;">`darkgrey`</span>.
	pub const DARK_GREY: Self = Self {
		red:   0.662745098,
		green: 0.662745098,
		blue:  0.662745098,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darkkhaki;">`darkkhaki`</span>.
	pub const DARK_KHAKI: Self = Self {
		red:   0.741176471,
		green: 0.717647059,
		blue:  0.419607843,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darkmagenta;">`darkmagenta`</span>.
	pub const DARK_MAGENTA: Self = Self {
		red:   0.545098039,
		green: 0.0,
		blue:  0.545098039,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darkolivegreen;">`darkolivegreen`</span>.
	pub const DARK_OLIVE_GREEN: Self = Self {
		 red:   0.333333333,
		 green: 0.419607843,
		 blue:  0.184313725,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: darkorange;">`darkorange`</span>.
	pub const DARK_ORANGE: Self = Self {
		red:   1.0,
		green: 0.549019608,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darkorchid;">`darkorchid`</span>.
	pub const DARK_ORCHID: Self = Self {
		red:   0.6,
		green: 0.196078431,
		blue:  0.8,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darkred;">`darkred`</span>.
	pub const DARK_RED: Self = Self {
		red:   0.545098039,
		green: 0.0,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darksalmon;">`darksalmon`</span>.
	pub const DARK_SALMON: Self = Self {
		red:   0.913725490,
		green: 0.588235294,
		blue:  0.478431373,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darkseagreen;">`darkseagreen`</span>.
	pub const DARK_SEA_GREEN: Self = Self {
		red:   0.560784314,
		green: 0.737254902,
		blue:  0.560784314,
		alpha: 1.0,
	};

	/// The web colour <span style="color: darkslateblue;">`darkslateblue`</span>.
	pub const DARK_SLATE_BLUE: Self = Self {
		 red:   0.282352941,
		 green: 0.239215686,
		 blue:  0.545098039,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: darkslategrey;">`darkslategrey`</span>.
	pub const DARK_SLATE_GREY: Self = Self {
		 red:   0.184313725,
		 green: 0.309803922,
		 blue:  0.309803922,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: darkturquoise;">`darkturquoise`</span>.
	pub const DARK_TURQUOISE: Self = Self {
		 red:   0.0,
		 green: 0.807843137,
		 blue:  0.819607843,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: darkviolet;">`darkviolet`</span>.
	pub const DARK_VIOLET: Self = Self {
		red:   0.580392157,
		green: 0.0,
		blue:  0.827450980,
		alpha: 1.0,
	};

	/// The web colour <span style="color: deeppink;">`deeppink`</span>.
	pub const DEEP_PINK: Self = Self {
		red:   1.0,
		green: 0.078431373,
		blue:  0.576470588,
		alpha: 1.0,
	};

	/// The web colour <span style="color: deepskyblue;">`deepskyblue`</span>.
	pub const DEEP_SKY_BLUE: Self = Self {
		 red:   0.0,
		 green: 0.749019608,
		 blue:  1.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: dimgrey;">`dimgrey`</span>.
	pub const DIM_GREY: Self = Self {
		red:   0.411764706,
		green: 0.411764706,
		blue:  0.411764706,
		alpha: 1.0,
	};

	/// The web colour <span style="color: dodgerblue;">`dodgerblue`</span>.
	pub const DODGER_BLUE: Self = Self {
		 red:   0.117647059,
		 green: 0.564705882,
		 blue:  1.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: firebrick;">`firebrick`</span>.
	pub const FIRE_BRICK: Self = Self {
		red:   0.698039216,
		green: 0.133333333,
		blue:  0.133333333,
		alpha: 1.0,
	};

	/// The web colour <span style="color: floralwhite;">`floralwhite`</span>.
	pub const FLORAL_WHITE: Self = Self {
		red:   1.0,
		green: 0.980392157,
		blue:  0.941176471,
		alpha: 1.0,
	};

	/// The web colour <span style="color: forestgreen;">`forestgreen`</span>.
	pub const FOREST_GREEN: Self = Self {
		 red:   0.133333333,
		 green: 0.545098039,
		 blue:  0.133333333,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: fuchsia;">`fuchsia`</span>.
	pub const FUCHSIA: Self = Self {
		red:   1.0,
		green: 0.0,
		blue:  1.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: gainsboro;">`gainsboro`</span>.
	pub const GAINSBORO: Self = Self {
		red:   0.862745098,
		green: 0.862745098,
		blue:  0.862745098,
		alpha: 1.0,
	};

	/// The web colour <span style="color: ghostwhite;">`ghostwhite`</span>.
	pub const GHOST_WHITE: Self = Self {
		red:   0.972549020,
		green: 0.972549020,
		blue:  1.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: gold;">`gold`</span>.
	pub const GOLD: Self = Self {
		red:   1.0,
		green: 0.843137255,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: goldenrod;">`goldenrod`</span>.
	pub const GOLDENROD: Self = Self {
		red:   0.854901961,
		green: 0.647058824,
		blue:  0.125490196,
		alpha: 1.0,
	};

	/// The web colour <span style="color: grey;">`grey`</span>.
	pub const GREY: Self = Self {
		red:   0.501960784,
		green: 0.501960784,
		blue:  0.501960784,
		alpha: 1.0,
	};

	/// The web colour <span style="color: green;">`green`</span>.
	pub const GREEN: Self = Self {
		 red:   0.0,
		 green: 0.501960784,
		 blue:  0.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: greenyellow;">`greenyellow`</span>.
	pub const GREEN_YELLOW: Self = Self {
		red:   0.678431373,
		green: 1.0,
		blue:  0.184313725,
		alpha: 1.0,
	};

	/// The web colour <span style="color: honeydew;">`honeydew`</span>.
	pub const HONEYDEW: Self = Self {
		red:   0.941176471,
		green: 1.0,
		blue:  0.941176471,
		alpha: 1.0,
	};

	/// The web colour <span style="color: hotpink;">`hotpink`</span>.
	pub const HOT_PINK: Self = Self {
		red:   1.0,
		green: 0.411764706,
		blue:  0.705882353,
		alpha: 1.0,
	};

	/// The web colour <span style="color: indianred;">`indianred`</span>.
	pub const INDIAN_RED: Self = Self {
		red:   0.803921569,
		green: 0.360784314,
		blue:  0.360784314,
		alpha: 1.0,
	};

	/// The web colour <span style="color: indigo;">`indigo`</span>.
	pub const INDIGO: Self = Self {
		 red:   0.294117647,
		 green: 0.0,
		 blue:  0.509803922,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: ivory;">`ivory`</span>.
	pub const IVORY: Self = Self {
		red:   1.0,
		green: 1.0,
		blue:  0.941176471,
		alpha: 1.0,
	};

	/// The web colour <span style="color: khaki;">`khaki`</span>.
	pub const KHAKI: Self = Self {
		red:   0.941176471,
		green: 0.901960784,
		blue:  0.549019608,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lavender;">`lavender`</span>.
	pub const LAVENDER: Self = Self {
		red:   0.901960784,
		green: 0.901960784,
		blue:  0.980392157,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lavenderblush;">`lavenderblush`</span>.
	pub const LAVENDER_BLUSH: Self = Self {
		red:   1.0,
		green: 0.941176471,
		blue:  0.960784314,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lawngreen;">`lawngreen`</span>.
	pub const LAWN_GREEN: Self = Self {
		red:   0.486274510,
		green: 0.988235294,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lemonchiffon;">`lemonchiffon`</span>.
	pub const LEMON_CHIFFON: Self = Self {
		red:   1.0,
		green: 0.980392157,
		blue:  0.803921569,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightblue;">`lightblue`</span>.
	pub const LIGHT_BLUE: Self = Self {
		red:   0.678431373,
		green: 0.847058824,
		blue:  0.901960784,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightcoral;">`lightcoral`</span>.
	pub const LIGHT_CORAL: Self = Self {
		red:   0.941176471,
		green: 0.501960784,
		blue:  0.501960784,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightcyan;">`lightcyan`</span>.
	pub const LIGHT_CYAN: Self = Self {
		red:   0.878431373,
		green: 1.0,
		blue:  1.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightgoldenrodyellow;">`lightgoldenrodyellow`</span>.
	pub const LIGHT_GOLDENROD_YELLOW: Self = Self {
		red:   0.980392157,
		green: 0.980392157,
		blue:  0.823529412,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightgreen;">`lightgreen`</span>.
	pub const LIGHT_GREEN: Self = Self {
		red:   0.564705882,
		green: 0.933333333,
		blue:  0.564705882,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightgrey;">`lightgrey`</span>.
	pub const LIGHT_GREY: Self = Self {
		red:   0.827450980,
		green: 0.827450980,
		blue:  0.827450980,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightpink;">`lightpink`</span>.
	pub const LIGHT_PINK: Self = Self {
		red:   1.0,
		green: 0.713725490,
		blue:  0.756862745,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightsalmon;">`lightsalmon`</span>.
	pub const LIGHT_SALMON: Self = Self {
		red:   1.0,
		green: 0.627450980,
		blue:  0.478431373,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightseagreen;">`lightseagreen`</span>.
	pub const LIGHT_SEA_GREEN: Self = Self {
		 red:   0.125490196,
		 green: 0.698039216,
		 blue:  0.666666667,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: lightskyblue;">`lightskyblue`</span>.
	pub const LIGHT_SKY_BLUE: Self = Self {
		red:   0.529411765,
		green: 0.807843137,
		blue:  0.980392157,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightslategrey;">`lightslategrey`</span>.
	pub const LIGHT_SLATE_GREY: Self = Self {
		red:   0.466666667,
		green: 0.533333333,
		blue:  0.6,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightsteelblue;">`lightsteelblue`</span>.
	pub const LIGHT_STEEL_BLUE: Self = Self {
		red:   0.690196078,
		green: 0.768627451,
		blue:  0.870588235,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lightyellow;">`lightyellow`</span>.
	pub const LIGHT_YELLOW: Self = Self {
		red:   1.0,
		green: 1.0,
		blue:  0.878431373,
		alpha: 1.0,
	};

	/// The web colour <span style="color: lime;">`lime`</span>.
	pub const LIME: Self = Self {
		 red:   0.0,
		 green: 1.0,
		 blue:  0.0,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: limegreen;">`limegreen`</span>.
	pub const LIME_GREEN: Self = Self {
		 red:   0.196078431,
		 green: 0.803921569,
		 blue:  0.196078431,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: linen;">`linen`</span>.
	pub const LINEN: Self = Self {
		red:   0.980392157,
		green: 0.941176471,
		blue:  0.901960784,
		alpha: 1.0,
	};

	/// The web colour <span style="color: magenta;">`magenta`</span>.
	pub const MAGENTA: Self = Self {
		red:   1.0,
		green: 0.0,
		blue:  1.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: maroon;">`maroon`</span>.
	pub const MAROON: Self = Self {
		red:   0.501960784,
		green: 0.0,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: mediumaquamarine;">`mediumaquamarine`</span>.
	pub const MEDIUM_AQUAMARINE: Self = Self {
		red:   0.4,
		green: 0.803921569,
		blue:  0.666666667,
		alpha: 1.0,
	};

	/// The web colour <span style="color: mediumblue;">`mediumblue`</span>.
	pub const MEDIUM_BLUE: Self = Self {
		 red:   0.0,
		 green: 0.0,
		 blue:  0.803921569,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: mediumorchid;">`mediumorchid`</span>.
	pub const MEDIUM_ORCHID: Self = Self {
		red:   0.729411765,
		green: 0.333333333,
		blue:  0.827450980,
		alpha: 1.0,
	};

	/// The web colour <span style="color: mediumpurple;">`mediumpurple`</span>.
	pub const MEDIUM_PURPLE: Self = Self {
		red:   0.576470588,
		green: 0.439215686,
		blue:  0.858823529,
		alpha: 1.0,
	};

	/// The web colour <span style="color: mediumseagreen;">`mediumseagreen`</span>.
	pub const MEDIUM_SEA_GREEN: Self = Self {
		 red:   0.235294118,
		 green: 0.701960784,
		 blue:  0.443137255,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: mediumslateblue;">`mediumslateblue`</span>.
	pub const MEDIUM_SLATE_BLUE: Self = Self {
		red:   0.482352941,
		green: 0.407843137,
		blue:  0.933333333,
		alpha: 1.0,
	};

	/// The web colour <span style="color: mediumspringgreen;">`mediumspringgreen`</span>.
	pub const MEDIUM_SPRING_GREEN: Self = Self {
		 red:   0.0,
		 green: 0.980392157,
		 blue:  0.603921569,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: mediumturquoise;">`mediumturquoise`</span>.
	pub const MEDIUM_TURQUOISE: Self = Self {
		 red:   0.282352941,
		 green: 0.819607843,
		 blue:  0.8,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: mediumvioletred;">`mediumvioletred`</span>.
	pub const MEDIUM_VIOLET_RED: Self = Self {
		red:   0.780392157,
		green: 0.082352941,
		blue:  0.521568627,
		alpha: 1.0,
	};

	/// The web colour <span style="color: midnightblue;">`midnightblue`</span>.
	pub const MIDNIGHT_BLUE: Self = Self {
		 red:   0.098039216,
		 green: 0.098039216,
		 blue:  0.439215686,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: mintcream;">`mintcream`</span>.
	pub const MINT_CREAM: Self = Self {
		red:   0.960784314,
		green: 1.0,
		blue:  0.980392157,
		alpha: 1.0,
	};

	/// The web colour <span style="color: mistyrose;">`mistyrose`</span>.
	pub const MISTY_ROSE: Self = Self {
		red:   1.0,
		green: 0.894117647,
		blue:  0.882352941,
		alpha: 1.0,
	};

	/// The web colour <span style="color: moccasin;">`moccasin`</span>.
	pub const MOCCASIN: Self = Self {
		red:   1.0,
		green: 0.894117647,
		blue:  0.709803922,
		alpha: 1.0,
	};

	/// The web colour <span style="color: navajowhite;">`navajowhite`</span>.
	pub const NAVAJO_WHITE: Self = Self {
		red:   1.0,
		green: 0.870588235,
		blue:  0.678431373,
		alpha: 1.0,
	};

	/// The web colour <span style="color: navy;">`navy`</span>.
	pub const NAVY: Self = Self {
		 red:   0.0,
		 green: 0.0,
		 blue:  0.501960784,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: oldlace;">`oldlace`</span>.
	pub const OLD_LACE: Self = Self {
		red:   0.992156863,
		green: 0.960784314,
		blue:  0.901960784,
		alpha: 1.0,
	};

	/// The web colour <span style="color: olive;">`olive`</span>.
	pub const OLIVE: Self = Self {
		red:   0.501960784,
		green: 0.501960784,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: olivedrab;">`olivedrab`</span>.
	pub const OLIVE_DRAB: Self = Self {
		red:   0.419607843,
		green: 0.556862745,
		blue:  0.137254902,
		alpha: 1.0,
	};

	/// The web colour <span style="color: orange;">`orange`</span>.
	pub const ORANGE: Self = Self {
		red:   1.0,
		green: 0.647058824,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: orangered;">`orangered`</span>.
	pub const ORANGE_RED: Self = Self {
		red:   1.0,
		green: 0.270588235,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: orchid;">`orchid`</span>.
	pub const ORCHID: Self = Self {
		red:   0.854901961,
		green: 0.439215686,
		blue:  0.839215686,
		alpha: 1.0,
	};

	/// The web colour <span style="color: palegoldenrod;">`palegoldenrod`</span>.
	pub const PALE_GOLDENROD: Self = Self {
		red:   0.933333333,
		green: 0.909803922,
		blue:  0.666666667,
		alpha: 1.0,
	};

	/// The web colour <span style="color: palegreen;">`palegreen`</span>.
	pub const PALE_GREEN: Self = Self {
		red:   0.596078431,
		green: 0.984313725,
		blue:  0.596078431,
		alpha: 1.0,
	};

	/// The web colour <span style="color: paleturquoise;">`paleturquoise`</span>.
	pub const PALE_TURQUOISE: Self = Self {
		red:   0.686274510,
		green: 0.933333333,
		blue:  0.933333333,
		alpha: 1.0,
	};

	/// The web colour <span style="color: palevioletred;">`palevioletred`</span>.
	pub const PALE_VIOLET_RED: Self = Self {
		red:   0.858823529,
		green: 0.439215686,
		blue:  0.576470588,
		alpha: 1.0,
	};

	/// The web colour <span style="color: papayawhip;">`papayawhip`</span>.
	pub const PAPAYA_WHIP: Self = Self {
		red:   1.0,
		green: 0.937254902,
		blue:  0.835294118,
		alpha: 1.0,
	};

	/// The web colour <span style="color: peachpuff;">`peachpuff`</span>.
	pub const PEACH_PUFF: Self = Self {
		red:   1.0,
		green: 0.854901961,
		blue:  0.725490196,
		alpha: 1.0,
	};

	/// The web colour <span style="color: peru;">`peru`</span>.
	pub const PERU: Self = Self {
		red:   0.803921569,
		green: 0.521568627,
		blue:  0.247058824,
		alpha: 1.0,
	};

	/// The web colour <span style="color: pink;">`pink`</span>.
	pub const PINK: Self = Self {
		red:   1.0,
		green: 0.752941176,
		blue:  0.796078431,
		alpha: 1.0,
	};

	/// The web colour <span style="color: plum;">`plum`</span>.
	pub const PLUM: Self = Self {
		red:   0.866666667,
		green: 0.627450980,
		blue:  0.866666667,
		alpha: 1.0,
	};

	/// The web colour <span style="color: powderblue;">`powderblue`</span>.
	pub const POWDER_BLUE: Self = Self {
		red:   0.690196078,
		green: 0.878431373,
		blue:  0.901960784,
		alpha: 1.0,
	};

	/// The web colour <span style="color: purple;">`purple`</span>.
	pub const PURPLE: Self = Self {
		red:   0.501960784,
		green: 0.0,
		blue:  0.501960784,
		alpha: 1.0,
	};

	/// The web colour <span style="color: rebeccapurple;">`rebeccapurple`</span>.
	pub const REBECCA_PURPLE: Self = Self {
		red:   0.4,
		green: 0.2,
		blue:  0.6,
		alpha: 1.0,
	};

	/// The web colour <span style="color: red;">`red`</span>.
	pub const RED: Self = Self {
		red:   1.0,
		green: 0.0,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: rosybrown;">`rosybrown`</span>.
	pub const ROSY_BROWN: Self = Self {
		red:   0.737254902,
		green: 0.560784314,
		blue:  0.560784314,
		alpha: 1.0,
	};

	/// The web colour <span style="color: royalblue;">`royalblue`</span>.
	pub const ROYAL_BLUE: Self = Self {
		 red:   0.254901961,
		 green: 0.411764706,
		 blue:  0.882352941,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: saddlebrown;">`saddlebrown`</span>.
	pub const SADDLE_BROWN: Self = Self {
		red:   0.545098039,
		green: 0.270588235,
		blue:  0.074509804,
		alpha: 1.0,
	};

	/// The web colour <span style="color: salmon;">`salmon`</span>.
	pub const SALMON: Self = Self {
		red:   0.980392157,
		green: 0.501960784,
		blue:  0.447058824,
		alpha: 1.0,
	};

	/// The web colour <span style="color: sandybrown;">`sandybrown`</span>.
	pub const SANDY_BROWN: Self = Self {
		red:   0.956862745,
		green: 0.643137255,
		blue:  0.376470588,
		alpha: 1.0,
	};

	/// The web colour <span style="color: seagreen;">`seagreen`</span>.
	pub const SEA_GREEN: Self = Self {
		 red:   0.180392157,
		 green: 0.545098039,
		 blue:  0.341176471,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: seashell;">`seashell`</span>.
	pub const SEASHELL: Self = Self {
		red:   1.0,
		green: 0.960784314,
		blue:  0.933333333,
		alpha: 1.0,
	};

	/// The web colour <span style="color: sienna;">`sienna`</span>.
	pub const SIENNA: Self = Self {
		red:   0.627450980,
		green: 0.321568627,
		blue:  0.176470588,
		alpha: 1.0,
	};

	/// The web colour <span style="color: silver;">`silver`</span>.
	pub const SILVER: Self = Self {
		red:   0.752941176,
		green: 0.752941176,
		blue:  0.752941176,
		alpha: 1.0,
	};

	/// The web colour <span style="color: skyblue;">`skyblue`</span>.
	pub const SKY_BLUE: Self = Self {
		red:   0.529411765,
		green: 0.807843137,
		blue:  0.921568627,
		alpha: 1.0,
	};

	/// The web colour <span style="color: slateblue;">`slateblue`</span>.
	pub const SLATE_BLUE: Self = Self {
		red:   0.415686275,
		green: 0.352941176,
		blue:  0.803921569,
		alpha: 1.0,
	};

	/// The web colour <span style="color: slategrey;">`slategrey`</span>.
	pub const SLATE_GREY: Self = Self {
		red:   0.439215686,
		green: 0.501960784,
		blue:  0.564705882,
		alpha: 1.0,
	};

	/// The web colour <span style="color: snow;">`snow`</span>.
	pub const SNOW: Self = Self {
		red:   1.0,
		green: 0.980392157,
		blue:  0.980392157,
		alpha: 1.0,
	};

	/// The web colour <span style="color: springgreen;">`springgreen`</span>.
	pub const SPRING_GREEN: Self = Self {
		 red:   0.0,
		 green: 1.0,
		 blue:  0.498039216,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: steelblue;">`steelblue`</span>.
	pub const STEEL_BLUE: Self = Self {
		 red:   0.274509804,
		 green: 0.509803922,
		 blue:  0.705882353,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: tan;">`tan`</span>.
	pub const TAN: Self = Self {
		red:   0.823529412,
		green: 0.705882353,
		blue:  0.549019608,
		alpha: 1.0,
	};

	/// The web colour <span style="color: teal;">`teal`</span>.
	pub const TEAL: Self = Self {
		 red:   0.0,
		 green: 0.501960784,
		 blue:  0.501960784,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: thistle;">`thistle`</span>.
	pub const THISTLE: Self = Self {
		red:   0.847058824,
		green: 0.749019608,
		blue:  0.847058824,
		alpha: 1.0,
	};

	/// The web colour <span style="color: tomato;">`tomato`</span>.
	pub const TOMATO: Self = Self {
		red:   1.0,
		green: 0.388235294,
		blue:  0.278431373,
		alpha: 1.0,
	};

	/// The web colour <span style="color: turquoise;">`turquoise`</span>.
	pub const TURQUOISE: Self = Self {
		 red:   0.250980392,
		 green: 0.878431373,
		 blue:  0.815686275,
		 alpha: 1.0,
	};

	/// The web colour <span style="color: violet;">`violet`</span>.
	pub const VIOLET: Self = Self {
		red:   0.933333333,
		green: 0.509803922,
		blue:  0.933333333,
		alpha: 1.0,
	};

	/// The web colour <span style="color: wheat;">`wheat`</span>.
	pub const WHEAT: Self = Self {
		red:   0.960784314,
		green: 0.870588235,
		blue:  0.701960784,
		alpha: 1.0,
	};

	/// The web colour <span style="color: white;">`white`</span>.
	pub const WHITE: Self = Self {
		red:   1.0,
		green: 1.0,
		blue:  1.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: whitesmoke;">`whitesmoke`</span>.
	pub const WHITE_SMOKE: Self = Self {
		red:   0.960784314,
		green: 0.960784314,
		blue:  0.960784314,
		alpha: 1.0,
	};

	/// The web colour <span style="color: yellow;">`yellow`</span>.
	pub const YELLOW: Self = Self {
		red:   1.0,
		green: 1.0,
		blue:  0.0,
		alpha: 1.0,
	};

	/// The web colour <span style="color: yellowgreen;">`yellowgreen`</span>.
	pub const YELLOW_GREEN: Self = Self {
		red:   0.603921569,
		green: 0.803921569,
		blue:  0.196078431,
		alpha: 1.0,
	};
}
