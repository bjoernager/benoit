# About

[*Benoit*](https://mandelbrot.dk/benoit) is a free and open‐source Mandelbrot renderer written in Rust.

The project is structured around the main `benoit` library, of which front-ends can make use of.

The official front-ends currently include `benoit-cli` and `benoit-gui`, although the latter is currently not present in this repository.

# Usage

```
benoit-cli <path>
benoit-gui
```

The thread count may be specified using the environment variable `RAYON_NUM_THREADS`.

See [Docs.rs](https://docs.rs/benoit/latest/benoit/) for documentation.

# Mirrors

Benoit is officially hosted on the following mirrors:

* [mandelbrot.dk](https://mandelbrot.dk/benoit)
* [GitLab](https://gitlab.com/bjoernager/benoit)
* [GitHub](https://github.com/bjoernager/benoit)

# Copyright & License

Also see individual files for their licenses.

Note that the section *`benoit`* does **NOT** represent the entirety of the Benoit project, instead only the `benoit` library found in the `benoit` directory.

The contents of this readme are released under a Creative Commons Attribution-ShareAlike 4.0 International license, see <https://creativecommons.org/licenses/by-sa/4.0/> for more information.

## `benoit`

Copyright 2021, 2023-2024 Gabriel Bjørnager Jensen.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

## `benoit-cli` and `benoit-gui`

Copyright 2021, 2023-2024 Gabriel Bjørnager Jensen.

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
