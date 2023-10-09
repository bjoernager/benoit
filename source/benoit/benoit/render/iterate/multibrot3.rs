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

use crate::benoit::complex::Complex;

use crate::benoit::PRECISION;

extern crate rug;

use rug::Float;

pub fn multibrot3(z: &mut Complex, c: &Complex) {
	let za_temporary = z.real.clone(); // a

	//   (a+bi)^3
	// = (a+bi)(a+bi)(a+bi)
	// = (a^2-b^2+2abi)(a+bi)
	// = a^3+(a^2)bi-ab^2-(b^3)i+2(a^2)bi-2ab^2
	// = a^3+3(a^2)bi-3ab^2-(b^3)i
	//
	// <=> z_a = a^3-3ab^2
	//     z_b = 3(a^2)b-b^3

	let mut temporary0 = Float::with_val(PRECISION, &z.imag * &z.imag); // b^2

	let temporary1 = Float::with_val(PRECISION, &temporary0 * &z.imag); // b^3

	temporary0 *= &z.real; // ab^2
	temporary0 *= 0x3;     // 3ab^2

	z.real.square_mut(); // a^2

	z.imag *= &z.real; // (a^2)b

	z.real *= &za_temporary; // a^3
	z.real -= &temporary0;   // a^3-3ab^2
	z.real += &c.real;       // a^3-3ab^2+Re(c)

	z.imag *= 3.0;         // 3(a^2)b
	z.imag -= &temporary1; // 3(a^2)b-b^3
	z.imag += &c.imag;     // 3(a^2)b-b^3+Im(c)
}
