FROM rust:1.48.0

# Install GCC ARM
RUN apt update && apt -y install gcc-arm-none-eabi
