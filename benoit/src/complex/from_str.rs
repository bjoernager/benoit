/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit-cli.

	benoit-cli is free software: you can redistrib-
	ute it and/or modify it under the terms of the
	GNU General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit-cli is distributed in the hope that it
	will be useful, but WITHOUT ANY WARRANTY; with-
	out even the implied warranty of MERCHANTABILITY
	or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	General Public License for more details.

	You should have received a copy of the GNU Gene-
	ral Public License along with benoit-cli. If
	not, see <https://www.gnu.org/licenses/>.
*/

use crate::PRECISION;
use crate::complex::Complex;
use crate::error::Error;

use rug::Float;
use std::str::FromStr;

impl FromStr for Complex {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let seperator = {
			let skip: usize = match s.find('-') {
				Some(0x0) => 0x1,
				_         => 0x0,
			};

			let s = &s[skip..];

			if let Some(index) = s.find('+') {
				index + skip
			} else if let Some(index) = s.find('-') {
				index + skip
			} else {
				return Err(Error::BadComplexSeperator { expr: s.to_owned() });
			}
		};

		let unit = s.len() - 0x1;

		if s.find('i') != Some(unit) {
			return Err(Error::MissingImaginaryUnit { expr: s.to_owned() });
		};


		let real = Float::parse(&s[..seperator])?;
		let imag = Float::parse(&s[seperator..unit])?;

		Ok(Self::with_val(PRECISION, real, imag))
	}
}
