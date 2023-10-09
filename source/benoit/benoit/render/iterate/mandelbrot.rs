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

pub fn mandelbrot(z: &mut Complex, c: &Complex) {
	// The Mandelbrot Set (M) is defined as the set of
	// values in the complex plane where the iterating
	// function
	//
	// z(n+1) = z(n)^2+c
	//
	// stays bounded: I.e. the absolute value of (z) stays bounded:
	//
	// abs(z) = sqrt(Re(z)^2+Im(z)^2) <= 2^2 = 4.

	let za_temporary = z.real.clone(); // a

	// We can calculate the square of a complex number
	// as:
	//
	//   (a+bi)^2
	// = (a+bi)(a+bi)
	// = a^2+abi+abi-b^2
	// = a^2-b^2+2abi.

	z.real.square_mut();         // a^2
	z.real -= &z.imag * &z.imag; // a^2-b^2
	z.real += &c.real;           // a^2-b^2+Re(c)

	z.imag *= &za_temporary; // ab
	z.imag *= 2.0;           // 2ab
	z.imag += &c.imag;       // 2ab+Im(c)
}
