# 1. Prepare dependencies
FROM rust as planner
WORKDIR app
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir "src"
RUN echo "fn main() {println!(\"If you see this, the build broke\")}" > src/main.rs
RUN cargo install cargo-chef
RUN cargo chef prepare  --recipe-path recipe.json

# 2. Cache dependencies
FROM rust as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo install cargo-chef
RUN cargo chef cook --release --recipe-path recipe.json

# 3. Build application
FROM rust as builder
WORKDIR app
# Dependencies for proto
RUN apt update
RUN apt install protobuf-compiler -y
COPY . .
# Copy over the cached dependency files
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# 4. Final stage
FROM debian
COPY --from=builder /app/target/release/feedback_data_connection /usr/local/bin/feedback_data_connection
CMD ["feedback_data_connection"]
