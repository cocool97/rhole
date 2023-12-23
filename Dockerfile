########################
## Web interface builder
########################
FROM node:lts-alpine AS npm-builder

COPY rhole-app /home/rhole-app
WORKDIR /home/rhole-app
RUN yarn install
RUN NODE_OPTIONS=--openssl-legacy-provider yarn run build

#######################
## Rust backend builder
#######################
FROM rust:alpine AS rust-builder

RUN apk add --no-cache musl-dev

COPY . /home/rhole
WORKDIR /home/rhole
RUN cargo build --release --target x86_64-unknown-linux-musl

####################
## Final image build
####################
FROM alpine:3.18

RUN mkdir /etc/rhole
RUN mkdir /etc/rhole/web

COPY --from=npm-builder /home/rhole-app/build /etc/rhole/web
COPY --from=rust-builder /home/rhole/target/x86_64-unknown-linux-musl/release/rhole /usr/bin/rhole

ENV CONFIG_PATH=/etc/rhole/config.yml

CMD ["/usr/bin/rhole"]