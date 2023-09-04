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
use crate::benoit::render_data::RenderData;

extern crate rayon;
extern crate rug;

use rayon::prelude::*;
use rug::Float;
use std::sync::Arc;
use std::time::Instant;

impl Application {
	pub fn render(&self, iteration_count_buffer: &mut [u32], square_distance_buffer: &mut [f32], center_real: &Float, center_imaginary: &Float, zoom: &Float, maximum_iteration_count: u32) {
		eprint!("rendering...");

		let time_start = Instant::now();

		let iterator = self.iterator_function;

		let data = Arc::new(RenderData::new(iteration_count_buffer, square_distance_buffer, self.canvas_width, self.canvas_height, center_real.clone(), center_imaginary.clone(), zoom.clone(), maximum_iteration_count));

		(0x0..self.canvas_height).into_par_iter().for_each(|row| {
			Application::render_row(data.clone(), row, iterator);
		});

		let duration = time_start.elapsed();

		eprintln!(" done ({}ms)", duration.as_millis());
	}
}
