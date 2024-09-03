FROM rust:bullseye

RUN apt update && apt upgrade -y
RUN apt install strace -y

RUN curl https://wasmtime.dev/install.sh -sSf | bash

RUN cargo install cargo-component

WORKDIR /shenanigans

COPY . .

RUN cargo component build

EXPOSE 8080

CMD ["strace", "-e", "openat", "/root/.wasmtime/bin/wasmtime", "serve", "--dir=/shenanigans", "./target/wasm32-wasip1/debug/wasmtime_filesystem_debug.wasm"]