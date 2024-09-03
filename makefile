BINARY="target/wasm32-wasi/debug/wasmtime_filesystem_debug.wasm"

.PHONY: wasm
wasm:
	cargo component build

.PHONY: run
run:
	wasmtime run ${BINARY}

.PHONY: invoke
invoke:
	wasmtime run ${BINARY} --invoke run

.PHONY: serve
serve:
	wasmtime serve --dir=.::. ${BINARY}


.PHONY: bd
bd:
	docker build -f Dockerfile . -t local:wasm-serve

.PHONY: dr
dr:
	docker run -p 8080:8080 local:wasm-serve

.PHONY: shell
shell:
	docker run -it -p 8080:8080 local:wasm-serve /bin/bash
