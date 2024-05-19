#!/usr/bin/env sh

# install.sh
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

failure() {
	echo failure: ${1}
	exit 2
}

if [ -z "${1}" ]
then
	failure "build directory is not set"
fi

if [ -z "${2}" ]
then
	failure "binary directory is not set"
fi

sourceDirectory="${PWD}"
buildDirectory="${1}"
binaryDirectory="${2}"

#CARGO_TARGET_DIR="${buildDirectory}" cargo build --release

mkdir -pvm755 "${binaryDirectory}"

install -vm755 "${buildDirectory}/release/benoit-cli" "${binaryDirectory}/benoit-cli"
#install -vm755 "${buildDirectory}/release/benoit-gui" "${binaryDirectory}/benoit-gui"

if ! [ -z "${3}" ]
then
	shareDirectory="${3}"

	mkdir -pvm755 "${shareDirectory}/applications"
	mkdir -pvm755 "${shareDirectory}/icons"

	install -vm644 "icon.svg" "${shareDirectory}/icons/benoit.svg"
	#desktop-file-install --dir="${shareDirectory}/applications" "benoit.desktop"
fi
