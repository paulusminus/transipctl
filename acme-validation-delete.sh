#!/usr/bin/bash

podman run \
    -v transip-logs:/var/log/transip \
    --env CERTBOT_DOMAIN \
    --env TRANSIP_API_USERNAME \
    --secret transip-key \
    --name acme-validation-delete \
    --rm \
    localhost/paulusminus/transipctl \
    acme-validation-delete.transip
