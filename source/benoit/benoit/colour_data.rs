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

use crate::benoit::image::Image;
use crate::benoit::palette::PaletteData;

use std::mem::size_of;
use std::num::NonZeroUsize;
use std::ptr::addr_of;

pub struct ColourData {
	exponent:       f32,
	max_iter_count: u32,
	colour_range:   f32,

	palette_data: &'static PaletteData,

	image:  NonZeroUsize,
}

impl ColourData {
	#[must_use]
	pub fn new(
		image:          &Image,
		exponent:       f32,
		max_iter_count: u32,
		colour_range:   f32,
		palette_data:   &'static PaletteData,
	) -> ColourData {
		return ColourData {
			exponent:       exponent,
			max_iter_count: max_iter_count,
			colour_range:   colour_range,

			palette_data: palette_data,

			image: NonZeroUsize::new(image.data().as_ptr() as usize).unwrap(),
		};
	}

	#[must_use]
	pub fn index(&self, element: &(u8, u8, u8)) -> usize {
		let element_addr = addr_of!(*element) as usize;

		let index = (element_addr - self.image.get()) / size_of::<(u8, u8, u8)>();
		return index;
	}

	#[must_use]
	pub fn consts(&self) -> (f32, u32, f32, &'static PaletteData) {
		return (self.exponent, self.max_iter_count, self.colour_range, self.palette_data);
	}
}
