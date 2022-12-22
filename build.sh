#!/bin/sh

cargo build --release && mv -f target/thumbv7em-none-eabihf/release/pifm target/thumbv7em-none-eabihf/release/pifm.fap
