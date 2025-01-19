FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir "src"
RUN echo "fn main() {println!(\"If you see this, the build broke\")}" > src/main.rs
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM cacher AS builder
COPY . .
COPY --from=cacher /app/target target
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /app/target/release/feedback_review /usr/local/bin/feedback_review
CMD [ "feedback_review" ]
