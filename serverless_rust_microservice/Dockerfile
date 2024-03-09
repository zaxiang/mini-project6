FROM rust:latest as builder

WORKDIR /usr/src/my_serverless_microservice
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /usr/src/my_serverless_microservice

COPY --from=builder /usr/src/my_serverless_microservice/target/release/my_serverless_microservice .

EXPOSE 8080

CMD ["./my_serverless_microservice"]