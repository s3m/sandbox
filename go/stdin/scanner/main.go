package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"time"
)

func main() {
	stat, _ := os.Stdin.Stat()
	if (stat.Mode() & os.ModeCharDevice) == 0 {
		scanner := bufio.NewScanner(os.Stdin)
		scanner.Split(bufio.ScanBytes)
		chunk := make([]byte, 0, 1024)
		for scanner.Scan() {
			chunk = append(chunk, scanner.Bytes()...)
			if len(chunk) == 1024 {
				fmt.Printf("chunk: %s\n", chunk)
				chunk = nil
				time.Sleep(time.Second)
			}
		}
		if err := scanner.Err(); err != nil {
			log.Fatal(err)
		}
		fmt.Printf("last chunk: %s len: %d\n", chunk, len(chunk))
	} else {
		fmt.Fprintln(os.Stderr, "nothing in stdin, pipe somethinig")
		os.Exit(1)
	}
}
