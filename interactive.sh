#!/usr/bin/sh

podman run \
    -v transip-logs:/var/log/transip \
    --env CERTBOT_DOMAIN \
    --env CERTBOT_VALIDATION \
    --env TRANSIP_API_USERNAME \
    --secret transip-key \
    --name transipctl \
    -i \
    --rm paulmin.nl/certbot
