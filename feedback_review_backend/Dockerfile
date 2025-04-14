FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml .
RUN printf "\n[profile.release]\nlto = true\ncodegen-units = 1\n" >> Cargo.toml
RUN mkdir "src"
RUN echo "fn main() {println!(\"If you see this, the build broke\")}" > src/main.rs
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM chef AS builder
COPY . .
RUN printf "\n[profile.release]\nlto = true\ncodegen-units = 1\n" >> Cargo.toml
COPY --from=cacher /app/target target
RUN cargo build --release

FROM debian:latest
COPY --from=builder /app/target/release/feedback_review_backend /usr/local/bin/feedback_review_backend
CMD ["feedback_review_backend"]

