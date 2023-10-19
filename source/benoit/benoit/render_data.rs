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

use crate::benoit::width_height_ratio;
use crate::benoit::complex::Complex;

extern crate rug;

use rug::Float;
use std::mem::size_of;
use std::num::NonZeroUsize;
use std::ptr::addr_of;

pub struct RenderData {
	canvas_width: u32,

	centre: Complex,
	extra:  Complex,
	zoom:   Float,

	max_iter_count: u32,

	inverse: bool,
	julia:   bool,

	x_offset: f32,
	y_offset: f32,

	x_factor: f32,
	y_factor: f32,

	buffer: NonZeroUsize,
}

impl RenderData {
	#[must_use]
	pub fn new(
		buffer:         &[(u32, f32)],
		canvas_width:   u32,
		canvas_height:  u32,
		centre:         Complex,
		extra:          Complex,
		zoom:           Float,
		max_iter_count: u32,
		inverse:        bool,
		julia:          bool,
	) -> RenderData {
		let (width_ratio, height_ratio) = width_height_ratio(canvas_width, canvas_height);

		return RenderData {
			canvas_width: canvas_width,

			centre: centre,
			extra:  extra,
			zoom:   zoom,

			max_iter_count: max_iter_count,

			inverse: inverse,
			julia:   julia,

			x_offset: canvas_width  as f32 / -2.0,
			y_offset: canvas_height as f32 / -2.0,

			x_factor: 4.0 / canvas_width  as f32 * width_ratio,
			y_factor: 4.0 / canvas_height as f32 * height_ratio,

			buffer: NonZeroUsize::new(buffer.as_ptr() as usize).unwrap(),
		};
	}

	#[must_use]
	pub fn input(&self) -> (&Complex, &Complex, &Float, u32, bool, bool) {
		return (&self.centre, &self.extra, &self.zoom, self.max_iter_count, self.inverse, self.julia);
	}

	#[must_use]
	pub fn consts(&self) -> (f32, f32, f32, f32) {
		return (self.x_offset, self.y_offset, self.x_factor, self.y_factor);
	}

	#[must_use]
	pub fn coordinate(&self, element: &(u32, f32)) -> (u32, u32) {
		let element_addr = addr_of!(*element)   as usize;

		let index = (element_addr - self.buffer.get()) / size_of::<(u32, f32)>();

		let x = (index % self.canvas_width as usize) as u32;
		let y = (index / self.canvas_width as usize) as u32;
		return (x, y);
	}
}
