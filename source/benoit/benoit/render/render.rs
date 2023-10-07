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

use crate::benoit::render::{IteratorFunction, PointRenderer};
use crate::benoit::render::render_data::RenderData;

extern crate rayon;
extern crate rug;

use rayon::prelude::*;
use rug::Float;

pub fn render(iter_count_buffer: &mut [u32], square_dist_buffer: &mut [f32], canvas_width: u32, canvas_height: u32, centre_real: &Float, centre_imag: &Float, zoom: &Float, max_iter_count: u32, inverse: bool, point_renderer: PointRenderer, iterator: IteratorFunction) {
	let data = RenderData::new(iter_count_buffer, square_dist_buffer, canvas_width, canvas_height, centre_real.clone(), centre_imag.clone(), zoom.clone(), max_iter_count, inverse);

	let (canvas_size, overflow) = canvas_height.overflowing_mul(canvas_width);
	if overflow { panic!("overflow when calculating canvas size") };

	(0x0..canvas_size).into_par_iter().for_each(|index| {
		render_point(&data, index as u32, point_renderer, iterator);
	});
}

fn render_point(data: &RenderData, index: u32, point_renderer: PointRenderer, iterator: IteratorFunction) {
	let (iter_count_buffer, square_dist_buffer) = data.output_buffers();

	let (canvas_width, _) = data.canvas_size();

	let x = index % canvas_width;
	let y = index / canvas_width;

	let (iter_count, square_dist) = point_renderer(&data, x, y, iterator);

	// Sacrifice safety for speed by removing bounds-
	// checking.
	unsafe {
		*iter_count_buffer.get_unchecked_mut( index as usize) = iter_count;
		*square_dist_buffer.get_unchecked_mut(index as usize) = square_dist;
	}
}
