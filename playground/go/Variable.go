package main

import (
	"context"

	"github.com/Chara-X/github"
)

var Ctx = context.Background()
var Registry = github.Registry{
	Path:   "/home/6092004007@zte.intra/chara/codes/",
	Ignore: []string{"op-aif-wsm", "openpalette-chart-template", "apts", "secretary", "defense-test"},
}

func Push() { Registry.Push("https://github.com/Chara-X", "main") }
func Pull() { Registry.Pull("https://github.com/Chara-X", "main") }
