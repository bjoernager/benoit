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

pub fn burning_ship(z: &mut Complex, c: &Complex) {
	// The Burning Ship is different in that - during
	// iteration - the real and imaginary parts of (z)
	// are made absolute:
	//
	// z(n+1) = (abs(Re(z(n)))+i*abs(Im(z(n))))^2+c.

	z.real.abs_mut();                  // abs(a)
	let za_temporary = z.real.clone(); // abs(a)

	z.real.square_mut();         // abs(a)^2
	z.real -= &z.imag * &z.imag; // abs(a)^2-abs(b)^2
	z.real += &c.real;           // abs(a)^2-abs(b)^2+Re(c)

	z.imag.abs_mut();        // abs(b)
	z.imag *= &za_temporary; // abs(a)
	z.imag *= 2.0;           // 2*abs(a)
	z.imag += &c.imag;       // 2*abs(a)+Im(c)
}
