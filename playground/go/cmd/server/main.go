package main

import (
	"fmt"
	"net/http"
	"net/http/httputil"
)

func main() {
	var router = http.NewServeMux()
	router.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("Hello, World!"))
	})
	http.ListenAndServe("127.0.0.1:11019", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var reqDump, _ = httputil.DumpRequest(r, true)
		fmt.Println(string(reqDump))
		router.ServeHTTP(w, r)
	}))
}
