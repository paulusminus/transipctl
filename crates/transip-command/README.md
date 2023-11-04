![build-badge](https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![crates.io](https://img.shields.io/crates/d/transip-command)
![docs.rs](https://img.shields.io/docsrs/transip-command)

# transip-command

transip-command is a library that can parse a string and turn it in a command.


## Installation

```bash
cargo install transip-command
```

## Commands

- dns list \<domainname\>
- dns acme-challenge-delete \<domainname\>
- dns acme-challenge-set \<domainname\> \<acme-challenge\>
- domain list
- domain item \<domainname\>
- invoice list
- invoice item \<invoicenumber\>
- invoice pdf \<invoicenumber\>
- product list
- product elements \<productname\>

## Environment variable usage

You can use a enviroment variable to specify domainname, acme-challenge, invoicenumer or productname.

### Example

- domain item ${DOMAIN_NAME}
