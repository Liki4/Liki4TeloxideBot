FROM ubuntu:latest

RUN apt-get update && apt-get install bash

COPY target/release/liki4_teloxide_bot /bin/liki4_teloxide_bot

RUN chmod +x /bin/liki4_teloxide_bot

ENTRYPOINT ["/bin/liki4_teloxide_bot"]