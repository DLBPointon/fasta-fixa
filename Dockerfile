ARG RUST_VERSION=1.87
ARG APP_NAME=fasta_fixa

# Built in line with docker hardened images
# https://docs.docker.com/guides/rust/build-images/#:~:text=docker%2Drust%2Dhello-,Create%20a%20Dockerfile%20for%20Rust,few%20questions%20about%20your%20application.

#########
# BUILDER
#########

FROM docker.io/library/rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

# Build the application.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/

#########
# WORKER
#########

FROM docker.io/library/alpine:3.18 AS final
ARG APP_NAME

RUN apk update && apk add bash

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/$APP_NAME /bin/
