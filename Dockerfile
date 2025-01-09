FROM rust:1.83 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:12 AS final

WORKDIR /app

COPY --from=builder /app/target/release .

EXPOSE 3000

ENTRYPOINT [ "/app/ha-tester" ]
