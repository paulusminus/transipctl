# transipctl(1)

## NAME

transipctl --- Parse lines into commands to be executed on Transip Api Endpoint

## SYNOPSIS

`transipctl` [_options_]

## DESCRIPTION

This command takes lines from stdin or the filename provided. Each line is interpreted as a command to 
be executed on the Transip Api Endpoint. See [`transip-command`](https://crates.io/crates/transip-command) for all
list of available commands.

## OPTIONS

<dl>

<dt class="option-term" id="option-transipctl--v"><a class="option-anchor" href="#option-transipctl--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-transipctl---version"><a class="option-anchor" href="#option-transipctl---version"></a><code>--version</code></dt>
<dd class="option-desc">Print version info and exit. This option cannot be combined with other options.</dd>


<dt class="option-term" id="option-transipctl-filename"><a class="option-anchor" href="#option-transipctl-filename"></a><code>&lt;filename&gt;</code></dt>
<dd class="option-desc">Process the lines from &lt;filename&gt; and exit. This option cannot be combined with other options.</dd>


</dl>

## EXAMPLES

1. Display the version and exit:

       transipctl -v

2. Execute one command and exit:

       echo "domain list" | transipctl
