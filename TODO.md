This document specifies the current priorities of the project.

Please discuss with maintainers before adding new entries.

# High priority

## User-defined palettes

Allow for the end-user to specify custom palettes.

The primary problem is currently to decide how the interface should be?
Lua scripts, Python, scripts, webasm...?

## Magnet fractal

The fractal of the form

*z*<sub>*n*+1</sub> = ((*z*<sub>*n*</sub><sup>2</sup> + *c* - 1) / ((2*z*<sub>*n*</sub> + *c* - 2))<sup>2

should be defined with the `magnet` identifier.

## Multibrot Sets

Include more powers of the Multibrot Set fractals.
Main obstacle is writing them for use with Rug.
Also finish refactoring of `multibrot4`.

## Modular factorisers

The factorisers (see `benoit/src/render/render/plot.rs`) could be implemented similarly to palettese, etc.

Especially the interior factoriser I am *not* very proud about:

```rust
let factor = data.distance; // data: RawElement
```

# Normal priority

## More fractals

Find more fractals to be added.

Requirements include:

* New fractals should be easily recognisable and different from others
* Although self-similar, new fractals should not perfectly repeat themselves

## Rug replacement

Find a replacement for Rug, optimally a Rust-written project.

This goal depends on the assumption that floating-point operations are **not** necessary for us.

In the case that using floats instead of rationals etc., Rug *appears* to be the best solution, at the moment at least.

# Low priority

## Web front-end

Not currently necessary to include in the main repository.
Should have the identifier `benoit-web`.

## Interior colour parameters

Should the parameters used internally by `Colour` be RGBA (as they currently are) or another encoding, e.g. XYZA?
