#!/usr/bin/bash

podman run \
    -v transip-logs:/var/log/transip \
    --env CERTBOT_DOMAIN \
    --env TRANSIP_API_USERNAME \
    --secret transip-key \
    --name acme-validation-delete \
    --rm \
    docker.io/paulusminus/transipctl \
    acme-validation-delete.transip
