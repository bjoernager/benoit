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

use rug::{Assign, Float};

pub fn multibrot4(z: &mut Complex, c: &Complex) {
	//   (a+bi)^4
	// = (a+bi)^2*(a+bi)^2
	// = (a^2-b^2+2abi)^2
	// = (a^2-b^2+2abi)(a^2-b^2+2abi)
	// = a^4-(a^2)b^2+2(a^3)bi-(a^2)b^2+b^4-2a(b^3)i-4(a^2)b^2+2(a^3)bi-2a(b^3)i-4(a^2)b^2
	// = a^4-6(a^2)b^2+4(a^3)bi+b^4-4a(b^3)i
	//
	// <=> a = a^4-6(a^2)b^2+b^4
	//     b = 4(a^3)bi-4a(b^3)i

	let temporary0 = Float::with_val(PRECISION, &z.real * &z.real); // a^2
	let temporary1 = Float::with_val(PRECISION, &z.imag * &z.imag); // b^2

	let mut temporary2 = Float::with_val(PRECISION, &z.real * &z.imag); // ab
	temporary2 *= 4.0;                                                  // 4ab

	z.real.assign(&temporary0); // a^2
	z.real /= 6.0;              // a^2/6
	z.real -= &temporary1;      // a^2/6-b^2
	z.real *= &temporary0;      // a^4/6-(a^2)b^2
	z.real *= 6.0;              // a^4-6(a^2)b^2

	z.imag.assign(&temporary1); // b^2
	z.imag *= -1.0;             // -b^2
	z.imag += &temporary0;      // a^2-b^2
	z.imag *= temporary2;       // 4(a^3)b-4ab^3

	z.real += temporary1.square(); // a^4-6(a^2)b^2+b^4
	z.real += &c.real;             // a^4-6(a^2)b^2+b^4+Re(c)

	z.imag += &c.imag; // 4(a^3)b-4ab^3+Im(c)
}
