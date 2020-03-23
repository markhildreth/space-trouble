#!/bin/bash
if [ "$1" == "" ]; then
	echo "ERROR: Must supply device file (e.g., /dev/cu.usbmodem14401"
	echo ""
	exit 1
fi

set -e
cargo build --release --target="thumbv6m-none-eabi"
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/space_trouble target/thumbv6m-none-eabi/release/space_trouble.bin
bossac -p $1 -a -e -w -v -R --offset=0x2000 target/thumbv6m-none-eabi/release/space_trouble.bin
