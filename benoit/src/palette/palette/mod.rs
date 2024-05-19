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

mod data;

use crate::error::Error;

use enum_iterator::Sequence;
use std::str::FromStr;

/// The different types of palettes.
#[derive(Clone, Copy, Debug, Sequence)]
pub enum Palette {
	Simple,
	Twilight,
	Fire,
	Greyscale,
	Ruby,
	Emerald,
	Sapphire,
	Hsv,
	Lch,
	Ink,
	Mask,
	Thunder,
	Glacier,
}

impl Default for Palette {
	fn default() -> Self { Self::Fire }
}

impl FromStr for Palette {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		#[allow(clippy::enum_glob_use)]
		use Palette::*;

		match s {
			"emerald"   => Ok(Emerald),
			"fire"      => Ok(Fire),
			"greyscale" => Ok(Greyscale),
			"glacier"   => Ok(Glacier),
			"hsv"       => Ok(Hsv),
			"ink"       => Ok(Ink),
			"lch"       => Ok(Lch),
			"mask"      => Ok(Mask),
			"ruby"      => Ok(Ruby),
			"sapphire"  => Ok(Sapphire),
			"simple"    => Ok(Simple),
			"thunder"   => Ok(Thunder),
			"twilight"  => Ok(Twilight),

			_ => Err(Error::UnknownPalette { string: s.to_owned() })
		}
	}
}
