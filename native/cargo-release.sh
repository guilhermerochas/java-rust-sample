#!/usr/bin/env bash

rustup toolchain install nightly
cd native && rustup run nightly cargo build --release -Z unstable-options --out-dir ../src/main/resources