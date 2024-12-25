package main

import (
	"log"

	"github.com/gizak/termui/v3"
	"github.com/gizak/termui/v3/widgets"
)

func main() {
	if err := termui.Init(); err != nil {
		log.Fatalf("failed to initialize termui: %v", err)
	}
	defer termui.Close()
	var p = widgets.NewParagraph()
	p.Text = "Hello World!"
	p.SetRect(0, 0, 25, 5)
	termui.Render(p)
	for e := range termui.PollEvents() {
		if e.Type == termui.KeyboardEvent {
			break
		}
	}
}
