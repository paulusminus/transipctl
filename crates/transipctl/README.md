![`build-badge`]
[![`mit-badge`]](https://opensource.org/licenses/MIT)

# transipctl

transipctl is an executable that processes lines with transip api commands. It can take input from a file or stdin.

It uses the [transip](https://crates.io/crates/transip) and [transip-command](https://crates.io/crates/transip-command) library crates.

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

## Examples of environment variables in script files

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

[`build-badge`]: https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg
[`mit-badge`]: https://img.shields.io/badge/License-MIT-yellow.svg
