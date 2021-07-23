wasm: go.mod go.sum *.go
	docker run --rm -v ${PWD}:/src -w /src tinygo/tinygo:0.19.0 tinygo build -o policy.wasm -target=wasi -no-debug .

clean:
	rm policy.wasm
