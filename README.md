<center>
	<img src="https://fadaesen.dk/files/benoit.svg" style="width:16em" />
</center>

# benoit

[*benoit*](https://mandelbrot.dk/delta/benoit) is a free and open-source Mandelbrot renderer written in C++ aimed at producing accurate Mandelbrot renders at arbitrary positions in the set as fast as possible.

This speed is achieved by using as many of the host's avaialable CPUs as possible.

For information regarding copyright of the software and it's license, please read the *Copyright & License* section down below.

## Building

The included Makefile is supposed to work for most platforms.

In the event it doesn't, find a solution.

## Dependencies

This project depends on the following libraries:

* Boost.Multiprecision (for multiprecision calculations)
* {FMT} (for string-formatting)
* GMP (a dependency of Boost.Multiprecision)
* libpng (for encoding PNG images)
* libwebp (for encoding WebP images)
* MPFR (a dependency of Boost.Multiprecision)
* pugixml (for XML parsing)

The project is written for POSIX and therefore requires the POSIX libraries.

Other than that, it also requires the target system to have the LLP64 data-model or greater.

Targets with a pointer size of less than 64b are currently incompatible, but architectures like Aarch64, AMD64, IA-64 and PPC64 are expected to work, no-problem.

## Copyright & License

Copyright (c) 2021 Gabriel Jensen.

All rights reserved.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.

If not, see <https://www.gnu.org/licenses/>.

## Questions

### How can I donate?

Thanks for considering it. What can I say?

Sadly, I currently do not have any way for you to donate unless.

But again thanks for wanting to do so. Here, have some gløgg: 🍷

### Why 64b-only?

64b architectures use 32b to represent an `int`, while 32b architectures use 16b.

32b systems are being deprecated by a lot of entities, and therefore it makes life easier to just expect a 64b data-mode. Why even bother, right?

Well actually some embedded-systems use 32b processors, and to the user of those I say: *Suck my big duck!*

### Why *blah blah blah*?

Please ask a constructive question.

### Why “benoit”?

This project is named *benoit* in honour of Benoit Mandelbrot, the discoverer of the, you guessed it, Mandelbrot set.

Benoit was a cool dude, but most people remember him only under the name *Mandelbrot*, if at all.

I think Benoit is a cool name, and Mr. Mandelbrot was a cool dude, so why not make a cool programme named after two cool things?
