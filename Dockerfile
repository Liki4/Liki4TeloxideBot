FROM alpine:latest

RUN apk add --no-cache bash

COPY target/x86_64-unknown-linux-musl/release/liki4_teloxide_bot /bin/liki4_teloxide_bot

RUN chmod +x /bin/liki4_teloxide_bot

ENTRYPOINT ["/bin/liki4_teloxide_bot"]