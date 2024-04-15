FROM alpine:3.19

ENV TRANSIP_API_PRIVATE_KEY=/run/secrets/transip-key
ENV TRANSIP_API_LOG_DIR=/var/log/transip
ENV TRANSIP_API_TOKEN_PATH=/token.txt
ENV TRANSIP_API_WHITELISTED_ONLY=true
ENV TRANSIP_API_READONLY=false
ENV TRANSIP_API_TOKEN_EXPIRATION="5 minutes"
ENV RUST_LOG info

WORKDIR /

COPY target/x86_64-unknown-linux-musl/release/transipctl /
COPY crates/transipctl/scripts/acme-validation-delete.transip /
COPY crates/transipctl/scripts/acme-validation-set.transip /

ENTRYPOINT [ "/transipctl" ]
