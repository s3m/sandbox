package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"time"
)

func Chunks(data []byte, atEOF bool) (advance int, token []byte, err error) {
	if atEOF && len(data) == 0 {
		return 0, nil, nil
	}
	if len(data) >= 1024 {
		return 1024, data[0:1024], nil
	}
	return len(data), data[:], nil
}

func main() {
	start := time.Now()
	nBytes, nChunks := int64(0), int64(0)
	scanner := bufio.NewScanner(os.Stdin)
	buf := make([]byte, 0, 1<<20)
	scanner.Buffer(buf, 1<<20)
	scanner.Split(Chunks)
	for scanner.Scan() {
		chunk := scanner.Bytes()
		nBytes += int64(len(chunk))
		nChunks++
		time.Sleep(time.Second)
		fmt.Printf("chunk = %s\n", chunk)
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	elapsed := time.Since(start)
	log.Printf("Bytes: %d Chunks: %d: epapsed: %v\n", nBytes, nChunks, elapsed)
}
