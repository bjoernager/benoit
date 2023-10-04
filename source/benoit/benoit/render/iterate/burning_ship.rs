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

extern crate rug;

use rug::Float;

pub fn burning_ship(za: &mut Float, zb: &mut Float, ca: &Float, cb: &Float) {
	// The Burning Ship is different in that - during
	// iteration - the real and imaginary parts of (z)
	// are made absolute:
	//
	// z(n+1) = (abs(Re(z(n)))+i*abs(Im(z(n))))^2+c.

	za.abs_mut();
	zb.abs_mut();

	let za_temporary = za.clone();

	za.square_mut();
	*za -= &*zb * &*zb;
	*za += ca;

	*zb *= za_temporary;
	*zb *= 2.0;
	*zb += cb;
}
