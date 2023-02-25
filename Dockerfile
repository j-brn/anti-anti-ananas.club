FROM lukemathwalker/cargo-chef AS planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef AS builder
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:stable-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/anti-anti-ananas /usr/local/bin
COPY --from=builder /app/assets /app/assets
COPY --from=builder /app/messages.txt /app/messages.txt
ENTRYPOINT ["/usr/local/bin/anti-anti-ananas"]