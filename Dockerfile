FROM rust:latest as builder
WORKDIR /usr/src

RUN USER=root cargo new swavan-api-service
WORKDIR /usr/src/swavan-api-service
COPY Cargo.toml Cargo.lock ./

COPY src ./src
RUN cargo install --path .

FROM gcr.io/distroless/cc
COPY --from=builder /usr/local/cargo/bin/swavan-api-service .
USER 1000
EXPOSE 8080
CMD ["./swavan-api-service"]
