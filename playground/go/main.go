package main

//go:generate stringer -type=User
type User struct {
	ID   int
	Name string
}
type Foo struct {
	me *Foo
}

func main() {
	var f = Foo{}
	f.me = &f
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
