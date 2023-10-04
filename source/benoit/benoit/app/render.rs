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
use crate::benoit::render::render_data::RenderData;

extern crate rayon;
extern crate rug;

use rayon::prelude::*;
use rug::Float;
use std::sync::Arc;

impl App {
	pub fn render(&self, iter_count_buffer: &mut [u32], square_dist_buffer: &mut [f32], centre_real: &Float, centre_imag: &Float, zoom: &Float, max_iter_count: u32) {
		let row_renderer = self.row_renderer;
		let iterator     = self.iterator_function;

		let data = Arc::new(RenderData::new(iter_count_buffer, square_dist_buffer, self.canvas_width, self.canvas_height, centre_real.clone(), centre_imag.clone(), zoom.clone(), max_iter_count));

		(0x0..self.canvas_height).into_par_iter().for_each(|row| {
			row_renderer(data.clone(), row as u32, iterator);
		});
	}
}
