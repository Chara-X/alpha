package main

import (
	"bytes"
	"encoding/csv"
	"fmt"
	"log"
	"os"
)

func main() {
	var in, _ = os.ReadFile("test.csv")
	r := csv.NewReader(bytes.NewReader(in))
	records, err := r.ReadAll()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Print(records)
}
