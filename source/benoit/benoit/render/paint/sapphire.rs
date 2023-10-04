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

pub fn sapphire(factor: f32) -> (f32, f32, f32) {
	let factor = factor % 1.0;

	let factor = (if factor >= 1.0 / 2.0 {
		1.0 - factor
	} else {
		factor
	}) * 2.0;

	let (red, green, blue) = if !factor.is_nan() {
		(factor * factor * factor, factor * factor, factor)
	} else {
		(0.0, 0.0, 0.0)
	};

	return (red, green, blue);
}
