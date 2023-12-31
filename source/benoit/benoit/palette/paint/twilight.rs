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

// Palette function from mandelbrotsdl, my first
// Mandelbrot renderer.

pub fn twilight(factor: f32) -> (f32, f32, f32) {
	let factor = factor % 1.0;

	let red   = 9.0  * (1.0 - factor) * factor         * factor         * factor;
	let green = 15.0 * (1.0 - factor) * (1.0 - factor) * factor         * factor;
	let blue  = 8.5  * (1.0 - factor) * (1.0 - factor) * (1.0 - factor) * factor;

	return (red, green, blue);
}
