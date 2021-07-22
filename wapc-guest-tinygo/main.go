package main

import (
	"fmt"

	wapc "github.com/wapc/wapc-guest-tinygo"
)

func main() {
	wapc.RegisterFunctions(wapc.Functions{
		"hello": hello,
	})
}

func hello(payload []byte) ([]byte, error) {
	fmt.Printf("got %v\n", string(payload))
	wapc.HostCall("myBinding", "sample", "hello", []byte("Simon"))
	return []byte("Hello"), nil
}
