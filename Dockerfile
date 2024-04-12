FROM rust:1.77

RUN apt update
RUN apt install docker.io -y

WORKDIR /usr/src/docker-stat-exporter
COPY . .

RUN cargo install --path .

CMD ["docker-stat-exporter"]
