FROM rust:1.85-alpine3.21 AS builder

RUN apk update && apk add ca-certificates git openssl-dev gcc g++ libc-dev binutils openssl-libs-static

WORKDIR /opt
COPY Cargo.toml .
COPY src src
RUN cargo build --release

FROM alpine:3.21 AS runner


WORKDIR /opt
COPY --from=builder /opt/target/release/gateway .
CMD ["./gateway", "--port", "1000"]
