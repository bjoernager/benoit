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

use crate::benoit::fractal::Fractal;

pub mod allocate;
pub mod render;

pub struct Render {
	canvas_width:  u32,
	canvas_height: u32,

	info: Option<(Fractal, u32)>,

	iter_count_buffer:  Vec::<u32>,
	square_dist_buffer: Vec::<f32>,
}

impl Render {
	#[must_use]
	pub fn canvas_size(&self) -> (u32, u32) {
		return (self.canvas_width, self.canvas_height);
	}

	#[must_use]
	pub fn info(&self) -> Option<(Fractal, u32)> {
		return self.info.clone();
	}

	#[must_use]
	pub fn data<'a>(&'a self) -> (&'a Vec::<u32>, &'a Vec::<f32>) {
		return (&self.iter_count_buffer, &self.square_dist_buffer);
	}
}
