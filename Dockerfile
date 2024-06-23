FROM alpine:latest

ENV TRANSIP_API_PRIVATE_KEY=/etc/transip.pem
ENV TRANSIP_API_LOG_DIR=/var/log/transip
ENV TRANSIP_API_TOKEN_PATH=/root/token.txt
ENV TRANSIP_API_WHITELISTED_ONLY=true
ENV TRANSIP_API_READONLY=false
ENV TRANSIP_API_TOKEN_EXPIRATION="5 minutes"
ENV RUST_LOG trace

WORKDIR /root/

COPY target/x86_64-unknown-linux-musl/release/transipctl /usr/bin/
COPY crates/transipctl/scripts/acme-validation-delete.transip /usr/bin/
COPY crates/transipctl/scripts/acme-validation-set.transip /usr/bin/

ENTRYPOINT [ "/usr/bin/transipctl" ]
