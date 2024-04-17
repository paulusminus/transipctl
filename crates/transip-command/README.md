![build-badge](https://github.com/paulusminus/transipctl/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![docs.rs](https://img.shields.io/docsrs/transip-command)
[![Crates.io](https://img.shields.io/crates/v/transip-command)](https://crates.io/crates/transip-command)

# transip-command

transip-command is a library that can parse a string and turn it in a command. It has one optional feature propagation.

## Commands

- availibility-zones
- comment \<text\>
- dns acme-validation-delete \<domainname\>
- dns acme-validation-set \<domainname\> \<acme-validation\>
- dns delete \<domainname\> \<recordname\> \<ttl\> \<recordtype\> \<content\>
- dns insert \<domainname\> \<recordname\> \<ttl\> \<recordtype\> \<content\>
- dns list \<domainname\>
- domain list
- domain item \<domainname\>
- email-box list \<domainname\>
- email-box list \<domainname\> \<id\>
- email-forward list \<domainname\>
- email-forward item \<domainname\> \<id\>
- invoice list
- invoice item \<invoicenumber\>
- invoice pdf \<invoicenumber\>
- onerror print | exit
- ping
- product list
- product elements \<productname\>
- sleep \<number-of-seconds\>
- vps list
- vps item \<vpsname\>
- vps start \<vpsname\>
- vps stop \<vpsname\>
- vps reset \<vpsname\>
- vps lock \<vpsname\>
- vps unlock \<vpsname\>

### Available on feature propagation
- dns acme-validation-check \<domainname\> \<acme-validation\>

## Environment variable usage

You can use a enviroment variable to specify domainname, acme-validation or content.

### Examples

- domain item \${DOMAIN_NAME}
- dns acme-validation-set \${CERTBOT_DOMAIN} \${CERTBOT_VALIDATION}
