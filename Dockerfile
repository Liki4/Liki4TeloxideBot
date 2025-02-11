FROM ubuntu:latest

RUN apt-get update && apt-get install -y bash curl build-essential gcc fontconfig libfontconfig-dev libfreetype-dev

COPY target/release/liki4_teloxide_bot /bin/liki4_teloxide_bot

RUN chmod +x /bin/liki4_teloxide_bot

WORKDIR /bin/

ENTRYPOINT ["/bin/liki4_teloxide_bot"]