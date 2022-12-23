#!/bin/sh

cargo build --release && mv -f target/thumbv7em-none-eabihf/release/pifm-f0 target/thumbv7em-none-eabihf/release/pifm-f0.fap
