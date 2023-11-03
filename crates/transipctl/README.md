![build-badge](https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![crates.io](https://img.shields.io/crates/d/transipctl)

# transipctl

transipctl is an executable that processes lines with transip api commands. It can take input from a file or stdin.

It uses the [transip](https://crates.io/crates/transip) library crate.

## Installation

```bash
cargo install transipctl
```

## Examples of using transipctl in script files

### Example script delete acme challenge

```bash
#!/usr/bin/transipctl

dns acme-challenge-delete paulmin.nl
```

### Example script set acme challenge

```bash
#!/usr/bin/transipctl

dns acme-challenge-set paulmin.nl dlkjfkeig
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


### Example 1

```bash
#!/usr/bin/transipctl

dns acme-challenge-delete ${CERTBOT_DOMAIN}
```

### Example 2

```bash
#!/usr/bin/transipctl

dns acme-challenge-set ${CERTBOT_DOMAIN} ${CERTBOT_VALIDATION}
```

