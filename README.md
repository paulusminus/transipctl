![build-badge](https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg)
![mit-badge](https://img.shields.io/badge/license-MIT-blue.svg)

# transipctl

transipctl is an executable that processes lines with transip api commands. It can take input from a file or stdin.

It uses the [transip](https://crates.io/crates/transip) library crate.

## Installation

```bash
cargo install transipctl
```

## Examples of using transipctl in script files

- [script delete acme challenge](https://raw.githubusercontent.com/paulusminus/transipctl/main/acme-challenge-delete.transip)
- [script set acme challenge](https://raw.githubusercontent.com/paulusminus/transipctl/main/acme-challenge-set.transip)

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

