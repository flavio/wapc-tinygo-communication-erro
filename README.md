This repo contains the source code needed to reproduce a communication issue
between a waPC host and a Wasm module produced by TinyGo 0.19.0

This is the structure of the repository:

* `wapc-host`: a simple waPC host written in Rust, using the wasmtime provider
* `wapc-guest-tinygo`: a simple waPC guest written in Go. The code exposes a `hello`
  waPC function. They payload given to the `hello` function is printed to the
  stdout via `fmt.Print`, then the guest invokes a waPC callback exposed by the host.

## How to reproduce the issue

Go into the `wapc-host` directory and run following command:

```
cargo run -- ../wapc-guest-tinygo/policy-tinygo-0.18.0.wasm hello
```

This will produce the following output:

```
got this is a test
Guest 1 invoked 'myBinding->sample:hello' with payload of Simon
```

Then run the Wasm module built with TinyGo 0.19.0:

```
cargo run -- ../wapc-guest-tinygo/policy-tinygo-0.19.0.wasm hello
```

This time the program will fail with this error:

```
Error: Error(GuestCallFailure("No error message set for call failure"))
```

## Building the Go program from scratch

The Go example can be built using TinyGo by doing the following command
inside of the `wapc-guest-tinygo` directory:

```
make wasm
```

The builds requires on docker running. The version of TinyGo to use can be changed
via the Makefile
