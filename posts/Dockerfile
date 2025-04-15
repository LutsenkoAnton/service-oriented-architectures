FROM rust:1.85-alpine3.21 AS chef
WORKDIR /opt
RUN apk update && apk add ca-certificates git openssl-dev gcc g++ libc-dev binutils openssl-libs-static protobuf-dev
RUN cargo install cargo-chef

FROM chef AS planner

COPY Cargo.toml .
COPY src/main.rs src/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as cacher
COPY --from=planner /opt/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo install sqlx-cli

FROM chef AS builder

WORKDIR /opt
COPY Cargo.toml .
COPY src src
COPY proto proto
COPY build.rs .
COPY .sqlx .sqlx
ENV SQLX_OFFLINE true
COPY --from=cacher /opt/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

FROM alpine:3.21 AS runner

WORKDIR /opt
COPY --from=builder /opt/target/release/posts .
COPY --from=builder /usr/local/cargo/bin/sqlx .
COPY migrations migrations
COPY startup.sh .
CMD ["./startup.sh"]
