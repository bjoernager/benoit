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

pub fn tricorn(za: &mut Float, zb: &mut Float, ca: &Float, cb: &Float) {
	// The Tricorn is only different from the
	// Mandelbrot Set in that the conjugate of (z) is
	// used instead of just (z):
	//
	// z(n+1) = (Re(z(n))-Im(z(n))i)^2+c.

	let za_temporary = za.clone(); // a

	za.square_mut();    // a^2
	*za -= &*zb * &*zb; // a^2-b^2
	*za += ca;          // a^2

	*zb *= za_temporary;
	// We can negate the value by multiplying with
	// (-1). A multiplication can be saved, as
	//
	// a*2*(-1) = a*(-2).
	//
	// Thus, we may combine these two multiplications.
	*zb *= -2.0;
	*zb += cb;
}
