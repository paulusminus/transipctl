#!/bin/bash

/usr/bin/pandoc  crates/transipctl/transipctl.1.md -s -t plain -o crates/transipctl/transipctl.1.txt
/usr/bin/pandoc  crates/transipctl/transipctl.1.md -s -t man -o crates/transipctl/transipctl.1
