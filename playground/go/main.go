package main

import "fmt"

//go:generate stringer -type=User
type User struct {
	ID   int
	Name string
}
type Pill int

const (
	Placebo Pill = iota
	Aspirin
	Ibuprofen
	Paracetamol
	Acetaminophen = Paracetamol
)

func main() {
	fmt.Println(Aspirin)
}
func Div(x, y int) int {
	return x / y
}
func Try(f func()) (err error) {
	defer func() {
		err, _ = recover().(error)
	}()
	f()
	return
}
