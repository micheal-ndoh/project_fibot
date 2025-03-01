FROM rust:latest
3 2 6 56
WORKDIR /app

COPY Cargo.toml Cargo.toml ./

COPY . .

RUN rustup update stable
RUN cargo build --release
ENTRYPOINT ["/app/target/release/project_fibot"]
