###########################################################
## Builder stage - builds Rust `rhole` binary
###########################################################
FROM rust:latest AS BUILDER

# Prepares for source-code copies
RUN mkdir -p /build
WORKDIR /build
RUN rustup target add armv7-unknown-linux-musleabihf

# Install necessary dependencies
RUN apt update
RUN apt upgrade -y
RUN apt install -y gcc-arm-linux-gnueabihf binutils-arm-linux-gnueabihf musl-dev musl-tools

# Cross-compiles source code
COPY Cargo.lock /build
COPY Cargo.toml /build
COPY .cargo /build/.cargo
COPY src /build/src
RUN cargo build --release --target armv7-unknown-linux-musleabihf
###########################################################

###########################################################
## Image build stage - builds final docker image
###########################################################
FROM alpine:latest

RUN mkdir -p /data

COPY config.yml /data
COPY --from=BUILDER /build/target/armv7-unknown-linux-musleabihf/release/rhole /bin/

ENTRYPOINT [ "/bin/rhole" ]
###########################################################