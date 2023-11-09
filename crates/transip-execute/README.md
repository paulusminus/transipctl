![build-badge](https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![docs.rs](https://img.shields.io/docsrs/transip-execute)
[![Crates.io](https://img.shields.io/crates/v/transip-execute)](https://crates.io/crates/transip-execute)

# transip-execute

transip-execute is a library that executes commands on the Transip Api Endpoint.
It uses the [`transip-command`] and the [`transip`] crates. It has one optional feature propagation.
If enabled you can execute a "dns acme-validation-check \<domain-name\> \<acme-validation\> command.


## Usage

Set environment variables needed to configure the Client. See [`transip`] documentation.

[`transip-command`]: https://crates.io/crates/transip-command
[`transip`]: https://crates.io/crates/transip