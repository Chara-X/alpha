package main

import (
	"context"

	"github.com/Chara-X/github"
)

var Ctx = context.Background()
var Registry = github.Registry{}

func Push() { Registry.Push("https://github.com/Chara-X", "main") }
func Pull() { Registry.Pull("https://github.com/Chara-X", "main") }
