#!/usr/bin/bash

podman run \
    -v transip-logs:/var/log/transip \
    --env CERTBOT_DOMAIN \
    --env CERTBOT_VALIDATION \
    --env TRANSIP_API_USERNAME \
    --secret transip-key \
    --name acme-validation-set \
    --rm \
    localhost/paulusminus/transipctl \
    acme-validation-set.transip
