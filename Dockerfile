FROM rust:1.75

WORKDIR /app

COPY . .

RUN rustup update stable

RUN cargo build --release

ENTRYPOINT ["/app/target/release/project_fibot"]