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

use crate::keyframe::{Keyframe, KeyframeInterpolater};

use benoit::{log, PRECISION};
use benoit::complex::Complex;
use num_traits::Num;
use rug::Float;
use rug::ops::PowAssign;

impl Keyframe {
	/// Interpolates between `self` and the given other keyframe.
	///
	/// The returned interpolater can be used as an iterator.
	///
	/// The way in which the values are interpolated differ:
	/// The centre, seed, and colour range values are interpreted as linear functions of the form (*a* &middot; *b* + *c*).
	/// The zoom value, however, is interpreted as a growth function of the form (*a*<sup>*x*</sup> &middot; *b*).
	///
	/// # Panics
	///
	/// Panics if frame indices don't match.

	#[must_use]
	pub fn interpolate_between(&self, other: &Self, frame_count: u32) -> KeyframeInterpolater {
		assert!(self.frame  <= other.frame, "starting frame ({}) is after stopping frame ({})", self.frame, other.frame);
		assert!(other.frame <  frame_count, "stopping frame ({}) is after last frame ({})", other.frame, frame_count - 0x1);

		let first_possible_frame = 0x0;
		let last_possible_frame  = frame_count - 0x1;

		let max_iter_count_step = linear_step(
			(first_possible_frame, i64::try_from(self.max_iter_count).unwrap()),
			(last_possible_frame,  i64::try_from(other.max_iter_count).unwrap()),
		);

		let centre_step       = linear_step_complex(   (first_possible_frame, &self.centre),      (last_possible_frame, &other.centre));
		let seed_step         = linear_step_complex(   (first_possible_frame, &self.seed),        (last_possible_frame, &other.seed));
		let zoom_factor       = growth_factor_bigfloat((first_possible_frame, &self.zoom),        (last_possible_frame, &other.zoom));
		let colour_range_step = linear_step(           (first_possible_frame, self.colour_range), (last_possible_frame, other.colour_range));

		let mut interpolator = KeyframeInterpolater {
			frame:      Some(self.frame),
			last_frame: other.frame,

			max_iter_count: self.max_iter_count,
			centre:         self.centre.clone(),
			seed:           self.seed.clone(),
			zoom:           self.zoom.clone(),
			colour_range:   self.colour_range,

			max_iter_count_step,
			centre_step,
			seed_step,
			zoom_factor,
			colour_range_step,
		};

		// Skip to the starting frame:
		for _ in 0x0..self.frame {
			interpolator.advance_values();
		}

		log!(value, interpolator);
		interpolator
	}
}

#[must_use]
fn linear_step<T: Default + From<u32> + Num>((t0, f0): (u32, T), (t1, f1): (u32, T)) -> T {
	if t1 == t0 || f1 == f0 { return Default::default() }

	// A linear function `f` of time `t` can be defined
	// as:
	//
	// f(t) = at+b
	//
	// The starting value `b` is already known. We use
	// the following expression to get the slope coef-
	// ficient `a`:
	//
	// a = (t1-t0)/(t1-t0)
	//
	// wherein f1 and f0 are the last and first results
	// of `f` - respectively - and t1 and t0 are like-
	// wise the last and first timepoints.

	(f1 - f0) / (T::from(t1) - T::from(t0))
}

#[must_use]
fn linear_step_bigfloat((t0, f0): (u32, &Float), (t1, f1): (u32, &Float)) -> Float {
	if t1 == t0 || f1 == f0 { return Float::new(PRECISION) }

	let denominator = f64::from(t1) - f64::from(t0);

	let numerator = Float::with_val(PRECISION, f1 - f0);
	numerator / denominator
}

#[must_use]
fn linear_step_complex((t0, f0): (u32, &Complex), (t1, f1): (u32, &Complex)) -> Complex {
	let real_step = linear_step_bigfloat((t0, &f0.real), (t1, &f1.real));
	let imag_step = linear_step_bigfloat((t0, &f0.imag), (t1, &f1.imag));

	Complex { real: real_step, imag: imag_step }
}

#[must_use]
fn growth_factor_bigfloat((t0, f0): (u32, &Float), (t1, f1): (u32, &Float)) -> Float {
	if t1 == t0 || f1 == f0 { return Float::with_val(PRECISION, 0x1) }

	// The growth function `f` of time `t` is:
	//
	// f(t) = b*a^t
	//
	// wherein `b` is the starting value and `a` is the
	// growth factor.
	//
	// To isolate `a` we use:
	//
	// a = nroot(t1-t0,f1/f0)
	//
	// wherein `t0`, `t1`, `f1`, and `f0` are the first
	// and last values on the two dimensions (`t` and
	// `f`), altough in theory arbitrary.
	//
	// Because the following is true:
	//
	// nroot(n,m) = m^(1/n)
	//
	// the expression for `a` is simplified for use
	// with Rug:
	//
	// a = (t1/t0)^(1/(f1-f0))

	let exponent = 1.0 / (f64::from(t1) - f64::from(t0));

	let mut factor = Float::with_val(PRECISION, f1 / f0);
	factor.pow_assign(exponent);

	factor
}
