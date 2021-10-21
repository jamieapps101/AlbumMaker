#! /usr/bin/env bash

cargo build --release
RESOURCE_PATH=`pwd`/resources ./target/release/album_maker -d ./test_files 