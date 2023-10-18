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
use crate::benoit::render::Render;
use std::time::Instant;

impl App {
	pub(super) fn render(&self, render: &mut Render) {
		eprint!("rendering...");

		let time_start = Instant::now();

		render.render(
			self.fractal,
			&self.centre,
			&self.zoom,
			&self.extra,
			self.max_iter_count,
		);

		let render_time = time_start.elapsed();

		eprintln!(" {:.3}ms", render_time.as_micros() as f32 / 1000.0);
	}
}