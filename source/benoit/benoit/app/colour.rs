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
use crate::benoit::task::colour_data::ColourData;

extern crate rayon;

use rayon::prelude::*;
use std::sync::Arc;

impl App {
	pub fn colour(&self, buffer: &mut [u8], max_iter_count: u32, iter_count_buffer: &[u32], square_dist_buffer: &[f32]) {
		let data = Arc::new(ColourData::new(buffer, self.canvas_width, max_iter_count, self.colour_range, iter_count_buffer, square_dist_buffer));

		(0x0..self.canvas_width).into_par_iter().for_each(|row| {
			App::colour_row(data.clone(), row as u32);
		});
	}
}
