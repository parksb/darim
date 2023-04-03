# builder stage

FROM rust:1.67 AS builder

WORKDIR /root
COPY . .

RUN cargo build --release

# production stage

FROM ubuntu:latest

ENV DEBIAN_FRONTEND noninteractive

WORKDIR /root
COPY --from=builder /root/target/release/darim-api-gateway ./app

RUN apt-get update && apt-get install -y libssl-dev

CMD ["./app"]
