FROM rust:1.48.0

RUN apt update
RUN apt -y install gcc-arm-none-eabi
