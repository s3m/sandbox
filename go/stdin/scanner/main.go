package main

import (
	"bufio"
	"log"
	"os"
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
	nBytes, nChunks := int64(0), int64(0)
	scanner := bufio.NewScanner(os.Stdin)
	buf := make([]byte, 0, 1024)
	scanner.Buffer(buf, 10<<20)
	scanner.Split(Chunks)
	for scanner.Scan() {
		nBytes += int64(len(scanner.Bytes()))
		nChunks++
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	log.Println("Bytes:", nBytes, "Chunks:", nChunks)
}
