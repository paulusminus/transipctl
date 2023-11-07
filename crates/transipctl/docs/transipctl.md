# transipctl(1)

## NAME

transipctl --- Parse lines into commands to be executed on Transip Api Endpoint

## SYNOPSIS

`transipctl` [_options_]

## DESCRIPTION

Compile local packages and all of their dependencies.

## OPTIONS

<dl>

<dt class="option-term" id="option-transipctl--v"><a class="option-anchor" href="#option-transipctl--v"></a><code>-v</code></dt>
<dt class="option-term" id="option-transipctl---version"><a class="option-anchor" href="#option-transipctl---version"></a><code>--version</code></dt>
<dd class="option-desc">Print version info and exit.</dd>


</dl>

## EXAMPLES

1. Display the version and exit:

       transipctl -v

2. Execute one command and exit:

       echo "domain list" | transipctl
