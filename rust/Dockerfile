FROM rust:1.72-alpine as builder

ENV WORKDIR_NAME="rust-on-docker"
ENV BINARY_NAME="rust_primer"

LABEL MAINTAINER="jw59615+github@gmail.com"
LABEL RUST_VERSION="1.72-alpine"
LABEL LICENSE="All rights reserved for original code, inherited for open source packages and software used"

# builder code that creates binary image for release
RUN cargo new --bin --name ${WORKDIR_NAME} /${WORKDIR_NAME}
RUN mkdir /src

WORKDIR ${WORKDIR_NAME}
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

FROM alpine:latest
ENV WORKDIR_NAME="rust-on-docker"
ENV BINARY_NAME="rust_primer"
RUN apk update && apk update && apk add gpg

# comment this line for true production
RUN apk add alpine-zsh-config 

COPY --from=builder /${WORKDIR_NAME}/target/release/${BINARY_NAME} ./${BINARY_NAME}

CMD ["/bin/sh"]