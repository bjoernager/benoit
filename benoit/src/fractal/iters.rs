/*
	Copyright 2021, 2023-2024 Gabriel Bjørnager Jen-
	sen.

	This file is part of benoit.

	benoit is free software: you can redistribute it
	and/or modify it under the terms of the GNU Af-
	fero General Public License as published by the
	Free Software Foundation, either version 3 of
	the License, or (at your option) any later ver-
	sion.

	benoit is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	Affero General Public License for more details.

	You should have received a copy of the GNU Af-
	fero General Public License along with benoit.
	If not, see <https://www.gnu.org/licenses/>.
*/

use crate::PRECISION;
use crate::complex::Complex;

use rug::{Assign, Float};

pub(in super) fn iter_antibrot(z: &mut Complex, c: &Complex) {
	//   z(n + 1) = int z(n)^2 + c dz
	//
	//   let:
	//   za = Re z, zb = zb
	//   ca = Re c, cb = Im c
	//
	//   (za + zb * i)^(2 + 1) / (2 + 1) + (za + zb * i) * (ca + cb * i)
	// = (za + zb * i)^3 / 3 + (za + zb * i) * (ca + cb * i)
	// = (za + zb * i) * (za + zb * i) * (za + zb * i) / 3 + (za + zb * i) * (ca + cb * i)
	// = (za^3 + za^2 * zb * i + za^2 * zb * i - za * zb^2 + za^2 * zb * i - za * zb^2 - za * zb^2 - zb^3 * i) / 3 + za * ca + za * cb * i + zb * ca * i - zb * cb
	// = (za^3 + za^2 * zb * i * 3 - za * zb^2 * 3 - zb^3 * i) / 3 + za * ca + za * cb * i + zb * ca * i - zb * cb
	//
	//   a
	// = (za^3 - za * zb^2 * 3) / 3 + za * ca - zb * cb
	// = za * (za^2 - zb^2 * 3) / 3 + za * ca - zb * cb
	//
	//   b
	// = (za^2 * zb * 3 - zb^3) / 3 + za * cb + zb * ca
	// = zb * (za^2 * 3 - zb^2) / 3 + za * cb + zb * ca

	let temp_z = z.clone(); // z

	let za_square = z.real.clone().square(); // za^2
	let zb_square = z.imag.clone().square(); // zb^2

	let mut temp = Float::with_val(PRECISION, &zb_square * -0x3); // zb^2 * -3 = -(zb^2 * 3)
	temp += &za_square;                                           // za^2 - zb^2 * 3

	z.real *= &temp;                  // za^3 - za * zb^2 * 3
	z.real /= 0x3;                    // (za^3 - za * zb^2 * 3) / 3
	z.real += &temp_z.real * &c.real; // (za^3 - za * zb^2 * 3) / 3 + za * ca
	z.real -= &temp_z.imag * &c.imag; // (za^3 - za * zb^2 * 3) / 3 + za * ca - zb * cb

	temp.assign(&za_square * 0x3); // za^2 * 3
	temp -= &zb_square;            // za^2 * 3 - zb^2

	z.imag *= &temp;                  // za^2 * zb * 3 - zb^3
	z.imag /= 0x3;                    // (za^2 * zb * 3 - zb^3) / 3
	z.imag += &temp_z.real * &c.imag; // (za^2 * zb * 3 - zb^3) / 3 + za * cb
	z.imag += &temp_z.imag * &c.real; // (za^2 * zb * 3 - zb^3) / 3 + za * cb + zb * ca
}

pub(in super) fn iter_burning_ship(z: &mut Complex, c: &Complex) {
	//   z(n + 1) = (|Re z(n)| + |Im z(n)| * i)^2 + c
	//
	//   let:
	//   za = Re z, zb = Im z
	//   ca = Re c, cb = Im c
	//
	//   (|za| + |zb| * i)^2 + ca + cb * i
	// = (|za| + |zb| * i) * (|za| + |zb| * i) + ca + cb * i
	// = |za|^2 + |za| * |zb| * i + |za| * |zb| * i - |zb|^2 + ca + cb * i
	// = za^2 + |zb * za| * i * 2 - zb^2 + ca + cb * i
	//
	//   a
	// = za^2 - zb^2 + ca
	//
	//   b
	// = |za| * |zb| * 2 + cb
	// = |za * zb| * 2 + cb

	let temp = z.real.clone(); // za

	z.real.square_mut();         // za^2
	z.real -= &z.imag * &z.imag; // za^2 - zb^2
	z.real += &c.real;           // za^2 - zb^2 + ca

	z.imag *= &temp;   // za * zb
	z.imag.abs_mut();  // |za * zb|
	z.imag *= 0x2;     // |za * zb| * 2
	z.imag += &c.imag; // |za * zb| * 2 + cb
}

pub(in super) fn iter_mandelbrot(z: &mut Complex, c: &Complex) {
	//   z(n + 1) = z(n)^2 + c
	//
	//   let:
	//   za = Re z, zb = Im z
	//   ca = Re c, cb = Im c
	//
	//   (za + zb * i)^2 + ca + cb * i
	// = (za + zb * i) * (za + zb * i) + ca + cb * i
	// = za^2 + za * zb * i + za * zb * i - zb^2 + ca + cb * i
	// = za^2 + zb * za * i * 2 - zb^2 + ca + cb * i
	//
	//   a
	// = za^2 - zb^2 + ca
	//
	//   b
	// = za * zb * 2 + cb

	let temp = z.real.clone(); // za

	z.real.square_mut();         // za^2
	z.real -= &z.imag * &z.imag; // za^2 - zb^2
	z.real += &c.real;           // za^2 - zb^2 + ca

	z.imag *= &temp;   // zb * zb
	z.imag *= 0x2;     // za * zb * 2
	z.imag += &c.imag; // za * zb * 2 + cb
}

pub(in super) fn iter_multibrot3(z: &mut Complex, c: &Complex) {
	//   z(n + 1) = z(n)^3 + c
	//
	//   let:
	//   za = Re z, zb = zb
	//   ca = Re c, cb = Im c
	//
	//   (za + zb * i)^3 + ca + cb * i
	// = (za + zb * i) * (za + zb * i) * (za + zb * i) + ca + cb * i
	// = za^3 + za^2 * zb * i + za^2 * zb * i - za * zb^2 + za^2 * zb * i - za * zb^2 - za * zb^2 - zb^3 * i + ca + cb * i
	// = za^3 + za^2 * zb * i * 3 - za * zb^2 * 3 - zb^3 * i + ca + cb + i
	//
	//   a
	// = za^3 - za * zb^2 * 3 + ca
	// = za * (za^2 - zb^2 * 3) + ca
	//
	//   b
	// = za^2 * zb * 3 - zb^3 + cb
	// = zb * (za^2 * 3 - zb^2) + cb

	let za_square = z.real.clone().square(); // za^2
	let zb_square = z.imag.clone().square(); // zb^2

	let mut temp = Float::with_val(PRECISION, &zb_square * -0x3); // zb^2 * (-3) = -(zb^2 * 3)
	temp += &za_square;                                           // za^2 - zb^2 * 3

	z.real *= &temp;   // za^3 - za * zb^2 * 3
	z.real += &c.real; // za^3 - za * zb^2 * 3 + ca

	temp.assign(&za_square * 0x3); // za^2 * 3
	temp -= &zb_square;            // za^2 * 3 - zb^2

	z.imag *= &temp;   // za^2 * zb * 3 - zb^3
	z.imag += &c.imag; // za^2 * zb * 3 - zb^3 + cb
}

pub(in super) fn iter_multibrot4(z: &mut Complex, c: &Complex) {
	// TODO: This is the last iterator that needs to be
	// refactored.

	//   (a+bi)^4
	// = (a+bi)^2*(a+bi)^2
	// = (a^2-b^2+2abi)^2
	// = (a^2-b^2+2abi)(a^2-b^2+2abi)
	// = a^4-(a^2)b^2+2(a^3)bi-(a^2)b^2+b^4-2a(b^3)i-4(a^2)b^2+2(a^3)bi-2a(b^3)i-4(a^2)b^2
	// = a^4-6(a^2)b^2+4(a^3)bi+b^4-4a(b^3)i
	//
	// <=> a = a^4-6(a^2)b^2+b^4
	//     b = 4(a^3)bi-4a(b^3)i

	let temp0 = Float::with_val(PRECISION, &z.real * &z.real); // a^2
	let temp1 = Float::with_val(PRECISION, &z.imag * &z.imag); // b^2

	let mut temp2 = Float::with_val(PRECISION, &z.real * &z.imag); // ab
	temp2 *= 4.0;                                                  // 4ab

	z.real.assign(&temp0); // a^2
	z.real /= 6.0;              // a^2/6
	z.real -= &temp1;      // a^2/6-b^2
	z.real *= &temp0;      // a^4/6-(a^2)b^2
	z.real *= 6.0;              // a^4-6(a^2)b^2

	z.imag.assign(&temp1); // b^2
	z.imag *= -1.0;             // -b^2
	z.imag += &temp0;      // a^2-b^2
	z.imag *= temp2;       // 4(a^3)b-4ab^3

	z.real += temp1.square(); // a^4-6(a^2)b^2+b^4
	z.real += &c.real;             // a^4-6(a^2)b^2+b^4+Re(c)

	z.imag += &c.imag; // 4(a^3)b-4ab^3+Im(c)
}

pub(in super) fn iter_tricorn(z: &mut Complex, c: &Complex) {
	//   z(n + 1) = (Re z(n) - Im z(n) * i)^2 + c
	//
	//   let:
	//   za = Re z, zb = Im z
	//   ca = Re c, cb = Im c
	//
	//   (za - zb * i)^2 + ca + cb * i
	// = (za - zb * i) * (za - zb * i) + ca + cb * i
	// = za^2 - za * zb * i - za * zb * i - zb^2 + ca + cb * i
	// = za^2 - zb * za * i * 2 - zb^2 + ca + cb * i
	//
	//   a
	// = za^2 - zb^2 + ca
	//
	//   b
	// = cb - za * zb * 2

	let temp = z.real.clone(); // za

	z.real.square_mut();         // za^2
	z.real -= &z.imag * &z.imag; // za^2 - zb^2
	z.real += &c.real;           // za^2

	z.imag *= &temp;   // za * zb
	z.imag *= -2.0;    // za * zb * -2 = -(za * zb * 2)
	z.imag += &c.imag; // cb - za * zb * 2
}
