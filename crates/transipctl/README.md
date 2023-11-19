![`build-badge`]
[![`mit-badge`]](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/transipctl)](https://crates.io/crates/transipctl)

# transipctl

transipctl is an executable that processes lines with transip api commands. It can take input from a file or stdin.

It uses the [`transip-execute`] library crate.

If you want to install transipctl for all users you can find a debian or rpm package on [`github-releases`].
If you prefer containers, an image is available on [`docker-hub`].

## Examples of using transipctl in script files

### Example script delete acme challenge

```bash
#!/usr/bin/transipctl

onerror exit

dns acme-validation-delete paulmin.nl
```

### Example script set acme validation

```bash
#!/usr/bin/transipctl

onerror exit

dns acme-validation-set paulmin.nl dlkjfkeig
```

## Examples of environment variables in script files

### Example 1

```bash
#!/usr/bin/transipctl

onerror exit

dns acme-validation-delete ${CERTBOT_DOMAIN}
```

### Example 2

```bash
#!/usr/bin/transipctl

onerror exit

dns acme-validation-set ${CERTBOT_DOMAIN} ${CERTBOT_VALIDATION}
```

[`build-badge`]: https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg
[`docker-hub`]: https://hub.docker.com/r/paulusminus/transipctl

[`mit-badge`]: https://img.shields.io/badge/License-MIT-yellow.svg
[`github-releases`]: https://github.com/paulusminus/transipctl/releases/latest/
[`docker-hub`]: https://hub.docker.com/r/paulusminus/transipctl
[`transip-command`]: https://crates.io/crates/transip-command
[`transip-execute`]: https://crates.io/crates/transip-execute
