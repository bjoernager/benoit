# 12

* Render and draw in different passes
* Update colouring
* Actually remove old makefile

# 11

* Remove old makefile
* Optimise renderer
* Modulise code
* Check keyboard input (allow viewpoint movement)
* Update colouring

# 10

* Rewrite in Rust again
* Update gitignore
* Update readme
* Update changelog format
* Use git tagging for versioning

# ↋

* Drop *boost::multiprecision::mpfr_float* in favour of the standard type *::__float128* for multiprecision
* Create a prettier colour palette
* Use JSON instead of XML for configuration
* Automatically create a configuration file if one doesn't already exist
* Greatly improve render time

# ↊

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
