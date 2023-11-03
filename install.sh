#!/bin/bash

cargo build --release
sudo install target/release/transipctl /usr/bin/
