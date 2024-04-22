FROM rust:1.76.0
    MAINTAINER Harikrishnan Namboothiri <harikri@protonmail.com>

RUN mkdir /app && apt-get update; \
    apt-get install build-essential -y
WORKDIR /app

COPY . /app
COPY ./docker_resources/Rocket.toml /app/Rocket.toml

RUN cargo build --release && cargo install diesel_cli; \
    mv ./target/release/blog_web .; \
    rm -rf src node_modules target; \
    apt-get remove --purge build-essential -y
