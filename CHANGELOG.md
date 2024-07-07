# 3.0.4

## `benoit`

* Update package metadata

# 3.0.3

## `benoit`

* Fix version docs

# 3.0.2

* Add per-crate readmes
* Bump dependencies

## `benoit`

* Remove `num-traits` as depedency

# 3.0.1

* Set Cargo metadata

# 3.0.0

* Bump major version
* Rewrite project
* Update copyright years
* Improve commenting
* Update gitignore
* Avoid unsafety
* Divide project into library and executables (CLI and GUI, latter currently unsupported)
* Create new logo
* Add installation script
* Add documentation
* Configure lints
* Add to-do list

## `benoit`

* Support palettes with alpha
* Remove WebP-support (support only PNG frames)
* Add new palette(s): `ink`, `mask`, `thunder`, `glacier`
* Support interior palettes
* Export images with 16 bits per channel
* Use 64 (in reality 63) bits for iteration counts
* Use our own complex data type
* Add new fractal(s): `antibrot`
* Define our own error type

## `benoit-cli`

* Register terminate
* Rework CLI
* Rework configuration
* Support resuming animations
* Define our own error type

# 2.7.1

* Improve info logs
* Enumerate iterators

# 2.7.0

* Bump minor version
* Rework animations (support multiple variables)
* Rework terminal arguments
* Improve error handling (avoid panics and assertions)
* Don't validate configuration immediately
* Update messages
* Rename 'burningship' to 'burning_ship' in configuration

# 2.6.3

* Add help
* Update readme
* Fix configuration validation
* Fix 'julia' not being respected in configurations
* Improve error handling
* Add launcher structure
* Always set terminal title

# 2.6.2

* Reorder fractal kinds
* Optimise renderer
* Refactor code

# 2.6.1

* Update readme

# 2.6.0

* Clean up and refactor code
* Add new palette 'simple'
* Remove getters and setters for fractal
* Implement FromStr for types
* Update messages
* Set terminal title in script mode
* Depend on windows (Windows-only)
* Bump minor version

# 2.5.1

* Fix readme

# 2.5.0

* Make fill_palettes safe
* Rename 'ancient' palette to 'twilight'
* Update package description
* Add new palettes 'emerald' and 'ruby'
* Bump minor version

# 2.4.1

* Remove renderers in favour of the fractal type
* Fix default configuration
* Update rendering
* Greatly improve safety
* Remove ctor as dependency
* Modulise and clean up code

# 2.4.0

* Clean up and restructure code
* Add multibrot d=4 fractal
* Update messages
* Improve safety
* Add image and render types

# 2.3.0

* Bump minor version
* Re-enable window border
* Support translation when rendering Julias
* Rework key handling
* Update naming convention
* Update controls guide
* Add key for resetting viewport
* Support perturbation
* Refactor and modulise code structure
* Rework animations
* Update configuration
* Rework row colourisers as point colourisers
* Fix image file extensions
* Use our own complex type

# 2.2.0

* Bump minor version
* Rework row renderers as point renderers
* Support inverse fractals (toggle with left control)

# 2.1.1

* Update readme
* Add desktop entry

# 2.1.0

* Refactor code structure
* Pre-calculate palettes
* Depend on ctor
* Remove factoriser functions
* Depend on enum-iterator
* Bump minor version
* Bump dependency versions
* Fix control guide

# 2.0.0

* Bump major version
* Update controls
* Support non-square canvasses again
* Modulise and refactor code
* Draw textual feedback to window (enable with F1)
* Bump dependency versions
* Support multiple palette functions (reflect in configuration)
* Add multibrot3 fractal
* Improve commenting
* Check interactive input
* Remove dumping from interactive mode
* Fix image file extensions

# 1.2.1

* Fix readme

# 1.2.0

* Bump minor version
* Update logging
* Support PNG encoding (set using configuration, depend on png)
* Make window borderless
* Update readme
* Don't set scale from configuration
* Update feedback
* Support setting dump path from configuration
* Update controls
* Colour according to new maximum iteration count if less than previous
* Update default colour range
* Modulise code
* Also dump colour range

# 1.1.0

* Bump minor version
* Set colour range in configuration
* Don't draw feedback on Julia

# 1.0.0

* Use hexadecimal versioning (with major.minor.patch)
* Don't animate single frames
* Support enabling of Julia rendering from configuration
* Perform configuration checks
* Colour in thread pools
* Update naming convention
* Synchronise with screen refresh rate
* Rework logs and timings
* Fix zoom animation
* Panic on missing configuration
* Add new logo
* Restructure code
* Update colouring (change using controls)

# 23

* Optimise and refactor code
* Update colouring
* Log version and copyright
* Fix complex-to-cartesian conversions (and controls)
* Update translation feedback

# 22

* Fix Julia toggle messages
* Fix configuration parameter names
* Bump dependency versions

# 21

* Support offsets in viewport feedback
* Remove support for non-square canvasses (update configuration)

# 20

* Draw positional feedback before renders (does not currently support offsets)
* Rename objects (allow some abbreviations)

# 1F

* Bring back Julia sets (using row renderers)
* Update controls (decrease sensitivity of zooms)
* Scale by default

# 1E

* Update colouring for small iteration counts
* Revert start zoom for interactive renders

# 1D

* Update colouring (smooth)
* Yield square distances from renders
* Update start zoom
* Add control for cycling fractals
* Add function for getting the name of a fractal

# 1C

* Optimise rendering
* Use Rayon for threading
* Update thread structure (use arc for common data)
* Use iterator functions instead of row renderers
* Fix render garbage (somehow)
* Lower precision

# 1B

* Support rendering of the Tricorn and Burning Ship fractals
* Update configuration
* Update start zoom value
* Remove Julia fractal(s)
* Bump dependency versions
* Update messages
* Update commenting

# 1A

* Support rendering of Julia sets
* Update controls
* Refactor code
* Update configuration

# 19

* Update controls guide (fix typo)

# 18

* Optimise renderer
* Update commenting

# 17

* Make configuration support more precise numbers (must be parsed as strings now)
* Use global constant for precision

# 16

* Use arbitrary-precision calculations
* Depend on Rug
* Optimise renderer
* Animate if configured
* Update commenting
* Remove scale option from configuration
* Auto-deduce thread count
* Update controls (only render on command)
* Update messages
* Refactor application structure
* Print controls

# 15

* Update controls
* Update configuration format
* Optimise renderer
* Fix thread count not being loaded

# 14

* Rename handle_key to handle_keys
* Only load configuration if provided

# 13

* Modulise code
* Check I/O errors
* Support configuration
* Depend on toml
* Update gitignore

# 12

* Clean up code
* Support rendering to files
* Depend on webp
* Modulise code

# 11

* Render using multiple threads

# 10

* Update colouring
* Rename changelog file: changelog.md => CHANGELOG.md

# F

* Update render message
* Add scaling setting

# E

* Render and draw in different passes
* Update colouring
* Actually remove old makefile

# D

* Remove old makefile
* Optimise renderer
* Modulise code
* Check keyboard input (allow viewpoint movement)
* Update colouring

# C

* Rewrite in Rust again
* Update gitignore
* Update readme
* Update changelog format
* Use git tagging for versioning

# B

* Drop *boost::multiprecision::mpfr_float* in favour of the standard type *::__float128* for multiprecision
* Create a prettier colour palette
* Use JSON instead of XML for configuration
* Automatically create a configuration file if one doesn't already exist
* Greatly improve render time

# A

* Fix #3
* Implement a working, multithreaded renderer forked from MandelbrotSDL
* Add more command-line options

# 9

* Remove *\*.ppm* from *.gitignore*
* Reformat changelog from HTML to Markdown
* Add an alpha channel to the image data
* Use *std::from_chars* instead of *std::stoi* in argument handler
* Inline the *benoit::wrtimg::iterwrt* lambda expression

# 8

* Fix #2

# 7

* Fix #1
* Fix some language mistakes
* Add a *Copyright & License* page in *README.md*
* Create logo
* Update *README.md*

# 6

* Actually switch compiler from Clang++ to G++
* Add more command-line arguments
* Move all data variables into seperate files
* Update *README.md*
* Remove PPM as a supported format
* Change maximum resolution to 65536
* Change maximum number of threads to 65536
* Fix WebP encoding
* Create temporary renderer that renders a *test* image using multiple threads

# 5

* Automatically detect number of threads
* Improve debugging
* Enable POSIX C
* Create foundation for loading and creating XML configurations
* Improve help screen
* Fix the static_assert in *include/benoit.hh* comparing to bits instead of bytes
* Switch compiler from Clang++ to G++ as it has better C++20 support
* Rework code structure
* Resize maximum resolution from *65535* to *4294967295*
* Make renderer able to create and use threads
* Create *purge* target in Makefile

# 4

* Remove build artifacts
* Update .gitignore to ignore more build artifacts

# 3

* Remove unused variables from Makefile
* Require the LLP64 data model when compiling
* Create working multithreaded example
* Depend also on libpng
* Remove JPEG as a supported image format

# 2

* Cleanup code
* Create argument handler
* Create better support for different image formats
* Remove C-string functions in favour of the ones in &ltcstring&gt
* Greatly improve debugging
* Create foundation for new multithreaded multiprecision renderer
* Remove old renderer
* Completely remove Rust artifacts

# 1

* Add changelog
* Add README
* Move codebase to C++
* Copy renderer from MandelbrotSDL
* Change default filetype to PPM

# 0

* First
