set -e
cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/space_team_irl target/thumbv6m-none-eabi/release/space_team_irl.bin
bossac -p /dev/cu.usbmodem1421 -e -w -v -R --offset=0x2000 target/thumbv6m-none-eabi/release/space_team_irl.bin
