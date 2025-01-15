package test

import (
	_ "io"
)

// [io.Copy]
// [ssh.Dial]
// [json.Decoder]
func Add(a, b int) int {
	// io.Copy(io.Discard, os.Stdout)
	return a + b
}
