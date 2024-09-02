# filesystem access check for a wasm:http module in serve mode

This is a wasi:http module that is built just by `cargo component build` with the present Cargo.toml file. 

## How to use this

1. Have docker installed
2. Clone the repository
3. build the docker image using `make bd` (bd is short for "build docker")
4. either run the container with `make rd` (rd is short for "run docker")
5. or shell into the container with `make shell`, and once inside you can do whatever. The image is based on rust:bullseye, it has wasmtime and strace installed into it

## Reason I know it can not access the filesystem

1. first, when I run the docker container and send a get request to localhost:8080, the response is an error, and the output in the docker container tells me thread panicked because can't open the directory
2. Using `strace -e openat` gives us the following output. Notice how there isn't an `openat` call for the `/shenanigans` directory as in the cli version

```
docker run -it local:wasm-serve /bin/bash
root@afeac1cb208d:/shenanigans# strace -e openat wasmtime serve --dir=/shenanigans target/wasm32-wasip1/debug/wasmtime_filesystem_debug.wasm
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libdl.so.2", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libgcc_s.so.1", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libpthread.so.0", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libm.so.6", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/lib/aarch64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/proc/self/maps", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/proc/self/cgroup", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/proc/self/mountinfo", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/sys/fs/cgroup/cpu.max", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "target/wasm32-wasip1/debug/wasmtime_filesystem_debug.wasm", O_RDONLY|O_CLOEXEC) = 9
openat(AT_FDCWD, "/root/.cache/wasmtime/modules/wasmtime-24.0.0/s8LhLMFfR3z6c3dWBqzMcZ5us-WI6qV3ytCgycPouZg", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/proc/self/cgroup", O_RDONLY|O_CLOEXEC) = 10
openat(AT_FDCWD, "/sys/fs/cgroup/cpu.max", O_RDONLY|O_CLOEXEC) = 10
openat(AT_FDCWD, "/root/.cache/wasmtime/modules/wasmtime-24.0.0/s8LhLMFfR3z6c3dWBqzMcZ5us-WI6qV3ytCgycPouZg.wip-atomic-write-mod", O_WRONLY|O_CREAT|O_EXCL|O_CLOEXEC, 0666) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/root/.cache/wasmtime/modules/wasmtime-24.0.0/s8LhLMFfR3z6c3dWBqzMcZ5us-WI6qV3ytCgycPouZg.wip-atomic-write-mod", O_WRONLY|O_CREAT|O_EXCL|O_CLOEXEC, 0666) = 10
Serving HTTP on http://0.0.0.0:8080/
<it is now waiting for an incoming request>
```
