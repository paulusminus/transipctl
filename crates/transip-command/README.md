![build-badge](https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![docs.rs](https://img.shields.io/docsrs/transip-command)
[![Crates.io](https://img.shields.io/crates/v/transip-command)](https://crates.io/crates/transip-command)

# transip-command

transip-command is a library that can parse a string and turn it in a command.


## Installation

```bash
cargo install transip-command
```

## Commands

- dns list \<domainname\>
- dns acme-validation-delete \<domainname\>
- dns acme-validation-set \<domainname\> \<acme-validation\>
- domain list
- domain item \<domainname\>
- invoice list
- invoice item \<invoicenumber\>
- invoice pdf \<invoicenumber\>
- onerror print | exit
- product list
- product elements \<productname\>
- sleep \<number-of-seconds\>

## Environment variable usage

You can use a enviroment variable to specify domainname, acme-validation.

### Examples

- domain item \${DOMAIN_NAME}
- dns acme-validation-set \${CERTBOT_DOMAIN} \${CERTBOT_VALIDATION}
