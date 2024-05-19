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

use crate::keyframe::Keyframe;

use benoit::complex::Complex;
use rug::Float;

/// Interpolater between keyframes.
///
/// This is done as an iterator over the interpolated values.
#[derive(Clone, Debug)]
pub struct KeyframeInterpolater {
	pub(in super) frame:      Option<u32>,
	pub(in super) last_frame: u32,

	pub(in super) max_iter_count: u64,
	pub(in super) centre:         Complex,
	pub(in super) seed:           Complex,
	pub(in super) zoom:           Float,
	pub(in super) colour_range:   f64,

	pub(in super) max_iter_count_step: i64,
	pub(in super) centre_step:         Complex,
	pub(in super) seed_step:           Complex,
	pub(in super) zoom_factor:         Float,
	pub(in super) colour_range_step:   f64,
}

impl KeyframeInterpolater {
	/// Advances the contained values to the next step.
	///
	/// # Panics
	///
	/// Panics if overflow occurs when calculating the new maximum iteration count.
	/// This is guaranteed to not happen as long as the iterator hasn't been completed.
	pub(in super) fn advance_values(&mut self) {
		self.max_iter_count = self.max_iter_count
			.checked_add_signed(self.max_iter_count_step)
			.unwrap();

		self.centre.real  += &self.centre_step.real;
		self.centre.imag  += &self.centre_step.imag;
		self.seed.real    += &self.seed_step.real;
		self.seed.imag    += &self.seed_step.imag;
		self.zoom         *= &self.zoom_factor;
		self.colour_range += self.colour_range_step;
	}
}

impl Iterator for KeyframeInterpolater {
	type Item = Keyframe;

	fn next(&mut self) -> Option<Self::Item> {
		let frame = self.frame?;
		assert!(frame <= self.last_frame);

		let keyframe = Keyframe {
			frame,
			max_iter_count: self.max_iter_count,
			centre:         self.centre.clone(),
			seed:           self.seed.clone(),
			zoom:           self.zoom.clone(),
			colour_range:   self.colour_range,
		};

		self.frame = (frame != self.last_frame).then(|| {
			self.advance_values();
			frame + 0x1
		});

		Some(keyframe)
	}
}
