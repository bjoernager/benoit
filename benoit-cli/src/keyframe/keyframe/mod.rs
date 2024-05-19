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

mod interpolate_between;

use crate::error::Error;

use benoit::PRECISION;
use benoit::complex::Complex;
use rug::Float;

/// Defines a single keyframe for animating.
///
/// Keyframes can be interpolated using the [`interpolate_between`](Keyframe::interpolate_between) method.
#[derive(Clone, Debug)]
pub struct Keyframe {
	pub frame:          u32,
	pub max_iter_count: u64,
	pub centre:         Complex,
	pub seed:           Complex,
	pub zoom:           Float,
	pub colour_range:   f64,
}

impl Keyframe {
	/// Validates `self`, wrapping it in a [`Result`].
	///
	/// The following requirements must be upheld:
	///
	/// * The maximum iteration count must be non-zero and less than or equal to `9223372036854775807`;
	/// * The zoom value must be positive and non-zero;
	/// * The colour range must be positive and non-zero;
	///
	/// If these are upheld, an [`Ok`] object is returned.
	///
	/// Do note that an "invalid" keyframe does not inherintly result in undefined behaviour if used.
	///
	/// # Errors
	///
	/// Yields an error if the keyframe could not be validated.
	pub fn validate(&self) -> Result<(), Error> {
		macro_rules! test {
			($assertion:expr, $($message:tt)*) => {{
				if !($assertion as bool) {
					return Err(Error::InvalidKeyframe { message: format!($($message)?) })
				}
			}};
		}

		test!(self.max_iter_count != 0x0, "max. iter. count ({}) cannot be zero", self.max_iter_count);

		// This is also tested by Config, but we don't know
		// if this keyframe even came from a configuration.
		test!(
			self.max_iter_count <= u64::try_from(i64::MAX).unwrap(),
			"max. iter. count ({}) cannot be greater than ({})",
			self.max_iter_count,
			i64::MAX,
		);

		test!(self.zoom > 0x0, "zoom ({}) must be greater than zero", self.zoom);

		test!(self.colour_range > 0.0, "colour range ({}) must be positive and non-zero", self.colour_range);

		Ok(())
	}
}

impl Default for Keyframe {
	fn default() -> Self { Self {
		frame:          0x0,
		max_iter_count: 0x100,
		centre:         Complex::new(PRECISION),
		seed:           Complex::new(PRECISION),
		zoom:           Float::with_val(PRECISION, 0x1),
		colour_range:   256.0,
	} }
}
