# BENOÎT

[*Benoit*](https://mandelbrot.dk/benoit) is a free and open‐source Mandelbrot renderer written in Rust. It is aimed at producing accurate renders at arbitrary positions in the set as fast as possible. Usage:

```
benoit [path]
```

… where *path* denotes the configuration file to read (optional). If no path is provided, the program is run in *interactive* mode, wherein the fractal is rendered in real‐time.

# Dependencies

Benoit makes use of the following external libraries:

* [enum-iterator](https://crates.io/crates/enum-iterator) for pre-calculating palettes
* [PNG](https://crates.io/crates/png) for encoding PNG images
* [Rayon](https://crates.io/crates/rayon) for threadpooling
* [Rug](https://crates.io/crates/rug) for multi‐precision
* [SDL2](https://crates.io/crates/sdl2) for interactive viewports
* [TOML](https://crates.io/crates/toml) for parsing TOML files
* [WebP](https://crates.io/crates/webp) for encoding WebP images

# Mirrors

Benoit is officially hosted on the following mirrors:

* [mandelbrot.dk](https://mandelbrot.dk/benoit)
* [GitLab](https://gitlab.com/bjoernager/benoit)
* [GitHub](https://github.com/bjoernager/benoit)

# Copyright & License

Copyright 2021, 2023 Gabriel Bjørnager Jensen.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
