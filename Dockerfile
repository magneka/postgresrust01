FROM ubuntu:22.04
 
RUN apt-get update && apt-get install -y curl
RUN apt-get install build-essential -y
RUN apt-get install pkg-config openssl libssl-dev mc -y  

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install trunk 

ENV CARGO_TARGET_DIR="/source/target"
