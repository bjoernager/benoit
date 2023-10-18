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

pub mod app;
pub mod colour_data;
pub mod complex;
pub mod configuration;
pub mod image;
pub mod fractal;
pub mod palette;
pub mod render;
pub mod render_data;
pub mod script;
pub mod video;

pub const VERSION: (u32, u32, u32) = (
	0x2, // Major
	0x5, // Minor
	0x1, // Patch
);

pub const PRECISION: u32 = 0x80;

pub const BAILOUT_DISTANCE: f32 = 256.0;

pub fn width_height_ratio(width: u32, height: u32) -> (f32, f32) {
	return if width > height {
		(1.0, height as f32 / width as f32)
	} else {
		(width as f32 / height as f32, 1.0)
	};
}
