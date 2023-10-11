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

pub fn hsv(factor: f32) -> (f32, f32, f32) {
	return hsv_to_rgb(factor, 7.0 / 8.0, 7.0 / 8.0);
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (f32, f32, f32) {
	return if saturation <= 0.0 {
		let value = value.min(1.0);

		(value, value, value)
	} else {
		let h = hue % 1.0 * 6.0;
		let s = saturation.min(1.0);
		let v = value.min(1.0);

		let f = h % 1.0;
		let p = v * (1.0 - s);
		let q = v * (1.0 - s * f);
		let t = v * (1.0 - s * (1.0 - f));

		match h.trunc() as u8 {
			0x0 => (v, t, p),
			0x1 => (q, v, p),
			0x2 => (p, v, t),
			0x3 => (p, q, v),
			0x4 => (t, p, v),
			0x5 => (v, p, q),
			_   => unreachable!(),
		}
	};
}
