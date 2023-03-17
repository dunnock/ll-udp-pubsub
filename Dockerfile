# syntax=docker/dockerfile:experimental
FROM rust:latest AS rust-build-stage
WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --example receive --target x86_64-unknown-linux-gnu
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release --example send --target x86_64-unknown-linux-gnu
RUN --mount=type=cache,target=/app/target \
    cp /app/target/x86_64-unknown-linux-gnu/release/examples/send /app/send
RUN --mount=type=cache,target=/app/target \
    cp /app/target/x86_64-unknown-linux-gnu/release/examples/receive /app/receive

FROM scratch AS export-stage

COPY --from=rust-build-stage /app/send /bin/send
COPY --from=rust-build-stage /app/receive /bin/receive
