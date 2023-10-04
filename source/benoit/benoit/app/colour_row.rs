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

use crate::benoit::app::App;
use crate::benoit::render::{FactoriserFunction, PaletteFunction};
use crate::benoit::render::colour_data::ColourData;

use std::sync::Arc;

impl App {
	pub fn colour_row(data: Arc<ColourData>, y: u32, factoriser: FactoriserFunction, palette_function: PaletteFunction) {
		let (iter_count_buffer, square_dist_buffer, image) = unsafe { data.slice(y) };

		for x in 0x0..data.canvas_width {
			let x = x as usize;

			let iter_count = unsafe { *iter_count_buffer.get_unchecked( x) };
			let distance   = unsafe { *square_dist_buffer.get_unchecked(x) }.sqrt();

			let factor = if iter_count < data.max_iter_count {
				factoriser(iter_count, distance, data.colour_range, data.exponent)
			} else {
				f32::NAN
			};

			let (red, green, blue) = palette_function(factor);

			let red   = (red   * 255.0).round() as u8;
			let green = (green * 255.0).round() as u8;
			let blue  = (blue  * 255.0).round() as u8;

			unsafe {
				let x = x * 0x3;

				*image.get_unchecked_mut(x)       = red;
				*image.get_unchecked_mut(x + 0x1) = green;
				*image.get_unchecked_mut(x + 0x2) = blue;
			}
		}
	}
}
