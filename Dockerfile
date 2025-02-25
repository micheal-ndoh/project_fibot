FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT ["/app/target/release/project_fibot"]
