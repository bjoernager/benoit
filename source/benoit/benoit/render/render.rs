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

use crate::benoit::complex::Complex;
use crate::benoit::fractal::{Fractal, IteratorFunction};
use crate::benoit::render::Render;
use crate::benoit::render_data::RenderData;
use crate::benoit::renderer::{PointRenderer, Renderer};

extern crate rayon;
extern crate rug;

use rayon::prelude::*;
use rug::Float;

impl Render {
	pub fn render(
		&mut self,
		fractal:            Fractal,
		renderer:           Renderer,
		centre:             &Complex,
		zoom:               &Float,
		extra:              &Complex,
		max_iter_count:     u32,
	) {
		assert!(max_iter_count > 0x0);

		let data = RenderData::new(
			&mut self.iter_count_buffer[..],
			&mut self.square_dist_buffer[..],
			self.canvas_width,
			self.canvas_height,
			centre.clone(),
			extra.clone(),
			zoom.clone(),
			max_iter_count,
			fractal.inverse(),
		);

		let canvas_size = self.canvas_height as usize * self.canvas_width as usize;

		let point_renderer = renderer.point_renderer();
		let iterator       = fractal.iterator();

		(0x0..canvas_size).into_par_iter().for_each(|index| {
			render_point(&data, index as usize, point_renderer, iterator);
		});

		self.info = Some((fractal, max_iter_count));
	}
}

fn render_point(data: &RenderData, index: usize, point_renderer: PointRenderer, iterator: IteratorFunction) {
	let (iter_count_buffer, square_dist_buffer) = unsafe { data.output_buffers() };

	let (canvas_width, _) = data.canvas_size();

	let x = (index % canvas_width as usize) as u32;
	let y = (index / canvas_width as usize) as u32;

	let (iter_count, square_dist) = point_renderer(&data, x, y, iterator);

	iter_count_buffer[ index as usize] = iter_count;
	square_dist_buffer[index as usize] = square_dist;
}
