package main

import (
	"embed"
	"testing"
)

// content holds our static web server content.
//
//go:embed cmd/* example/* "main.go"
var content embed.FS

func main() {
	testing.CoverMode()
}
func Add() {

}
