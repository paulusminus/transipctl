# transipctl(1)

## NAME

transipctl - Parse lines into commands to be executed on Transip Api Endpoint

## SYNOPSIS

`transipctl` [_options_] `-v` _input_

## DESCRIPTION

This command takes lines from stdin or the filename provided. Each line is interpreted as a command to 
be executed on the Transip Api Endpoint. See [`transip-command`](https://crates.io/crates/transip-command) for all
list of available commands.

## OPTIONS

{{#options}}

{{#option "`-v`"}}
Print version info and exit. This option cannot be combined with other options.
{{/option}}

{{#option "_input_"}}
The source input filename. Stdin is used if no filename is provided.
{{/option}}

{{/options}}

## EXAMPLES

1. Display the version and exit:

       transipctl -v

2. Execute one command and exit:

       echo "domain list" | transipctl
