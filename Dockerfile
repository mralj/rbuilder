#
# Base container (with sccache and cargo-chef)
#
# - https://github.com/mozilla/sccache
# - https://github.com/LukeMathWalker/cargo-chef
#
# Based on https://depot.dev/blog/rust-dockerfile-best-practices
#
FROM rust:1.81 as base

ARG FEATURES

RUN cargo install sccache --version ^0.8
RUN cargo install cargo-chef --version ^0.1

RUN apt-get update \
    && apt-get install -y clang libclang-dev

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_DIR=/sccache

#
# Planner container (running "cargo chef prepare")
#
FROM base AS planner
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.git ./.git
COPY ./crates/ ./crates/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

#
# Builder container (running "cargo chef cook" and "cargo build --release")
#
FROM base as builder
WORKDIR /app
# Default binary filename rbuilder
# Alternatively can be set to "reth-rbuilder" - to have reth included in the binary
ARG RBUILDER_BIN="rbuilder"
COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.git ./.git
COPY ./crates/ ./crates/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release --features="$FEATURES" --bin=${RBUILDER_BIN}

#
# Runtime container
#
FROM gcr.io/distroless/cc-debian12
WORKDIR /app

ARG RBUILDER_BIN="rbuilder"
COPY --from=builder /app/target/release/${RBUILDER_BIN} /app/rbuilder

ENTRYPOINT ["/app/rbuilder"]
