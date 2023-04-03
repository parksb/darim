# builder stage

FROM rust:1.67 AS builder

WORKDIR /root
COPY . .

RUN cargo build --release

# production stage

FROM ubuntu:latest

ENV DEBIAN_FRONTEND noninteractive

WORKDIR /root
COPY --from=builder /root/target/release/darim-server ./app

RUN apt-get update && apt-get install -y mysql-server mysql-client
RUN echo 'deb http://security.ubuntu.com/ubuntu xenial-security main' | tee /etc/apt/sources.list.d/xenial-security.list
RUN apt-get update && apt-get install -y libmysqlclient20
RUN ln -s /usr/lib/x86_64-linux-gnu/libmysqlclient.so.20.3.20 /usr/lib/libmysqlclient.so.20

CMD ["./app"]
