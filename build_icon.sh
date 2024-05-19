#!/usr/bin/env sh

# build_icon.sh
# Copyright 2024 Gabriel Bjørnager Jensen.
#
# Permission is hereby granted, free of charge, to
# any person obtaining a copy of this software and
# associated documentation files (the “Software”),
# to deal in the Software without restriction, in-
# cluding without limitation the rights to use,
# copy, modify, merge, publish, distribute, subli-
# cense, and/or sell copies of the Software, and
# to permit persons to whom the Software is fur-
# nished to do so, subject to the following con-
# ditions:
#
# The above copyright notice and this permission
# notice shall be included in all copies or sub-
# stantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WAR-
# RANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
# BUT NOT LIMITED TO THE WARRANTIES OF MER-
# CHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE
# AND NONINFRINGEMENT. IN NO EVENT SHALL THE AU-
# THORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
# CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
# ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
# FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
# OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

# Use in case an ICO image is wanted.
#
# On Arch, install `extra/icoutils` and `extra/
# imagemagick`.

failure() {
	echo failure: ${1}
	exit 2
}

if [ -z "${1}" ]
then
	failure "input name is not set"
fi

if [ -z "${2}" ]
then
	failure "output name is not set"
fi

renderDirectory="$(mktemp -d)"

render_icon() {
	name="${1}"
	width="${2}"

	output="${renderDirectory}/${name}_${width}x${width}.png"

	echo rendering \"${output}\"
	convert -background transparent "${name}.svg" -scale "${width}" "${output}"
}

input="${1}"

render_icon "${input}" 16
render_icon "${input}" 24
render_icon "${input}" 32
render_icon "${input}" 48
render_icon "${input}" 64
render_icon "${input}" 96
render_icon "${input}" 128
render_icon "${input}" 192
render_icon "${input}" 256

output="${2}.ico"

echo combining into \"${output}\"
icotool -co "${output}" "${renderDirectory}/"*".png"
