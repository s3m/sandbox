package main

import (
	"fmt"
	"os"
)

func main() {
	stat, _ := os.Stdin.Stat()
	if (stat.Mode() & os.ModeCharDevice) == 0 {
		fmt.Println("data is being piped to stdin")
	} else {
		fmt.Println("stdin is from a terminal")
	}
}
