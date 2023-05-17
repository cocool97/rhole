FROM alpine:3.17

RUN mkdir /etc/rhole
RUN mkdir /etc/rhole/web

COPY dist/* /etc/rhole/web
COPY target/x86_64-unknown-linux-musl/release/rhole /usr/bin/rhole

CMD ["/usr/bin/rhole", "start", "--config", "/etc/rhole/config.yml"]