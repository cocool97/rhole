###########################################################
## Builder stage - builds Rust `rhole` binary
###########################################################
FROM rust:latest AS BUILDER

# Prepares for source-code copies
RUN mkdir -p /build
WORKDIR /build
RUN rustup target add armv7-unknown-linux-gnueabihf 
RUN rustup toolchain install stable-armv7-unknown-linux-gnueabihf 

# Install necessary dependencies
RUN apt update
RUN apt upgrade -y
RUN apt install -y gcc-arm-linux-gnueabihf 
RUN apt install -y binutils-arm-linux-gnueabihf 
RUN apt install -y musl-dev 
RUN apt install -y musl-tools 
RUN apt install -y libssl-dev
RUN apt install -y g++-arm-linux-gnueabihf
RUN apt install -y libc6-dev-armhf-cross

# Cross-compiles source code
COPY Cargo.lock /build
COPY Cargo.toml /build
COPY .cargo /build/.cargo
COPY src /build/src
RUN cargo build --release --target armv7-unknown-linux-gnueabihf
###########################################################

###########################################################
## Image build stage - builds final docker image
###########################################################
FROM alpine:latest

RUN mkdir -p /data

COPY config.yml /data
COPY --from=BUILDER /build/target/armv7-unknown-linux-gnueabihf/release/rhole /bin/

ENTRYPOINT [ "/bin/rhole start --config /data/config.yml" ]
###########################################################