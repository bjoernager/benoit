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

use crate::benoit::application::Application;

impl Application {
	pub fn colour(&self, buffer: &mut [u8], iteration_count_buffer: &[u32], square_distance_buffer: &[f32]) {
		let canvas_size = self.canvas_height * self.canvas_width;

		for pixel in 0x0..canvas_size {
			let iteration_count = iteration_count_buffer[pixel as usize];

			let (red, green, blue) = if iteration_count != self.maximum_iteration_count {
				let distance = square_distance_buffer[pixel as usize].sqrt();

				let mut factor = (iteration_count as f32 + 1.0 - distance.log2().log2()) / 16.0 % 1.0;

				factor %= 1.0;

				factor = (if factor >= 1.0 / 2.0 {
					1.0 - factor
				} else {
					factor
				}) * 2.0;

				(factor * factor, factor * factor, factor)
			} else {
				(0.0, 0.0, 0.0)
			};

			let red   = (red   * 255.0).round() as u8;
			let green = (green * 255.0).round() as u8;
			let blue  = (blue  * 255.0).round() as u8;

			buffer[pixel as usize * 0x3]       = red;
			buffer[pixel as usize * 0x3 + 0x1] = green;
			buffer[pixel as usize * 0x3 + 0x2] = blue;
		}
	}
}
