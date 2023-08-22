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
	pub fn colour(&mut self, buffer: &mut [u8], data: &[u32]) {
		let canvas_size = self.canvas_height * self.canvas_width;

		for pixel in 0x0..canvas_size {
			let iteration_count = data[pixel as usize];

			let factor = {
				let factor = iteration_count as f32 / 64.0 % 1.0;

				(if factor >= 1.0 / 2.0 {
					1.0 - factor
				} else {
					factor
				}) * 2.0
			};

			let colour: u8 = if iteration_count != self.maximum_iteration_count {
				(factor * 255.0).round() as u8
			} else {
				0x0
			};

			buffer[pixel as usize * 0x3] = colour;
			buffer[pixel as usize * 0x3 + 0x1] = colour;
			buffer[pixel as usize * 0x3 + 0x2] = colour;
		}
	}
}
