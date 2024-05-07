# builder stage

FROM rust:1.67 AS builder

WORKDIR /root
COPY . .

RUN cargo build --release

# production stage

FROM ubuntu:latest

ENV DEBIAN_FRONTEND noninteractive
WORKDIR /root

RUN apt-get update && apt-get install -y mysql-server mysql-client

COPY --from=builder /root/target/release/darim-server ./app
CMD ["./app"]
