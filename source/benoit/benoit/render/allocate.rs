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

use crate::benoit::configuration::Configuration;
use crate::benoit::render::Render;

impl Render {
	pub fn allocate(width: u32, height: u32) -> Result<Render, String> {
		if width < Configuration::MIN_CANVAS_WIDTH || height < Configuration::MIN_CANVAS_WIDTH { return Err(format!("width and height may not be more than {}", Configuration::MIN_CANVAS_WIDTH)) };

		let (canvas_size, overflow) = (height as usize).overflowing_mul(width as usize);
		if overflow { return Err("overflow when calculating canvas size".to_string()) };

		let data: Vec<(u32, f32)> = vec![(0x0, 0.0); canvas_size];

		return Ok(Render {
			width:  width,
			height: height,

			info: None,

			data: data,
		});
	}
}
