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

use std::f32::consts::PI;

pub fn lch(factor: f32) -> (f32, f32, f32) {
	// Convert turns to radians:
	// 1 turn = 2pi radians

	let angle = factor * PI * 2.0;

	let (l, a, b) = lch_to_lab( 200.0 / 3.0, 1053.0 / 20.0, angle);
	let (x, y, z) = lab_to_xyz( l, a, b);
	let (r, g, b) = xyz_to_rgb( x, y, z);
	let (r, g, b) = rgb_to_srgb(r, g, b);

	return (r, g, b);
}

fn rgb_to_srgb(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
	fn srgb(value: f32) -> f32 {
		return if value > 7827.0 / 25000000.0 {
			211.0 / 200.0 * value.powf(5.0 / 12.0) - 11.0 / 200.0
		} else {
			298.0 / 25.0 * value
		};
	}

	let r = srgb(r);
	let g = srgb(g);
	let b = srgb(b);

	(r, g, b)
}

fn xyz_to_rgb(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
	let m: [[f32; 0x3]; 0x3] = [
		[
			12831.0 / 3959.0, -329.0 / 214.0, -1974.0 / 3959.0,
		],
		[
			-851781.0 / 878810.0, 1648619.0 / 878810.0, 36519.0 / 878810.0,
		],
		[
			705.0 / 12673.0, -2585.0 / 12673.0, 705.0 / 667.0,
		],
	];

	let r = m[0x0][0x0] * x + m[0x0][0x1] * y + m[0x0][0x2] * z;
	let g = m[0x1][0x0] * x + m[0x1][0x1] * y + m[0x1][0x2] * z;
	let b = m[0x2][0x0] * x + m[0x2][0x1] * y + m[0x2][0x2] * z;

	return (r, g, b);
}

fn lab_to_xyz(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
	let kappa:   f32 = 24389.0 / 27.0;
	let epsilon: f32 = 216.0 / 24389.0;

	let f1 = (l + 16.0) / 116.0;
	let f0 = a / 500.0 + f1;
	let f2 = f1 - b / 200.0;

	let temporary = (l + 16.0) / 116.0;

	let mut x = f0 * f0 * f0;
	let mut y = temporary * temporary * temporary;
	let mut z = f2 * f2 * f2;

	if x <= epsilon         { x = 1152.0 / 1195.0 * ((116.0 * f0 - 16.0) / kappa) };
	if l <= kappa * epsilon { y = l / kappa };
	if z <= epsilon         { z = 986.0 / 1195.0 * ((116.0 * f2 - 16.0) / kappa) };

	return (x, y, z);
}

fn lch_to_lab(l: f32, c: f32, h: f32) -> (f32, f32, f32) {
	let a = c * h.cos();
	let b = c * h.sin();

	return (l, a, b);
}
