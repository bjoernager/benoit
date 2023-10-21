/*
	Copyright 2021, 2023 Gabriel Bjørnager Jensen.

	This file is part of Benoit.

	Benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	Affero General Public License as published by
	the Free Software Foundation, either version 3
	of the License, or (at your option) any later
	version.

	Benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU
	Affero General Public License along with Benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::benoit::fractal::FractalKind;

use std::str::FromStr;

impl FromStr for FractalKind {
	type Err = String;

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		use FractalKind::*;

		let kind = match string {
			"burning_ship" => Some(BurningShip),
			"mandelbrot"   => Some(Mandelbrot),
			"multibrot3"   => Some(Multibrot3),
			"multibrot4"   => Some(Multibrot4),
			"tricorn"      => Some(Tricorn),
			_              => None,
		};

		return match kind {
			Some(kind) => Ok(kind),
			_          => Err(format!("invalid fractal kind \"{string}\"")),
		};
	}
}
