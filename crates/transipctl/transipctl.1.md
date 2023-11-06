---
title: transipctl
section: 1
header: User Manual
footer: transipctl
date: "0.2.1"
---

# NAME
transipctl - Command line utility to parse and run transip commands on the Transip Api Endpoint.

# SYNOPSIS
**transipctl** \[OPTIONS...\] \<FILENAME\>

# DESCRIPTION
**transipctl** is a command line utitity. Lines are read from stdin or from the FILENAME if specified. Each line is interpreted as a command that is to be executed on the Transip Api Endpoint.

If the Transip Api Endpoint returns data, the data will be serialized as YAML and displayed on stdout.

See https://crates.io/crates/transip-command for a list of supported commands.

# OPTIONS
**-v** **--version**
: display the name and version
