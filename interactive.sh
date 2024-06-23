#!/usr/bin/sh

docker run \
    -v transip-logs:/var/log/transip \
    --env CERTBOT_DOMAIN \
    --env CERTBOT_VALIDATION \
    --env TRANSIP_API_USERNAME \
    --mount type=bind,source=/etc/transip/home.pem,target=/etc/transip.pem,readonly \
    --name transipctl \
    -i \
    --tty \
    --rm \
    paulusminus/transipctl
