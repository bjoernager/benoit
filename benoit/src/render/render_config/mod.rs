/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit.

	benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU Af-
	fero General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU Af-
	fero General Public License along with benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::complex::Complex;
use crate::fractal::Fractal;

use rug::Float;

/// Used to configure renders.
///
/// This is expected by [`Render::generate`](crate::render::Render::generate), and also used by [`Render::plot`](crate::render::Render::plot).
#[derive(Clone, Debug)]
pub struct RenderConfig {
	/// The fractal to be rendered.
	pub fractal: Fractal,

	/// Whether to render the fractal's inverse or not.
	pub inverse: bool,

	/// Whether to render a Julia set or not.
	///
	/// If this is enabled, the `seed` field is used as the Julia set's point on the complex plane.
	/// In all other cases (i.e. it is disabled), `seed` is used as the starting value of *z*.
	pub julia: bool,

	/// The maximum ammount of iterations permitted.
	pub max_iter_count: u64,

	/// The centre value of the viewfinder, on the complex plane.
	pub centre: Complex,

	/// See `julia`.
	pub seed: Complex,

	/// The zoom level of the render.
	///
	/// Larger values converge the viewfinder on the value `centre`.
	pub zoom: Float,
}
