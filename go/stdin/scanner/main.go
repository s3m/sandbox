package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func ScanBytes(data []byte, atEOF bool) (advance int, token []byte, err error) {
	if atEOF && len(data) == 0 {
		return 0, nil, nil
	}
	if len(data) >= 4 {
		return 4, data[0:4], nil
	}
	return 0, data[0:], nil
}

func main() {
	nBytes, nChunks := int64(0), int64(0)
	scanner := bufio.NewScanner(os.Stdin)
	//buf := make([]byte, 0, 1024)
	//scanner.Buffer(buf, 10<<20)
	//	chunk := make([]byte, 0, 1024)
	scanner.Split(ScanBytes)
	for scanner.Scan() {
		fmt.Printf("scanner.Bytes() = %s\n", scanner.Bytes())
		//buf = append(buf, scanner.Bytes()...)
		nChunks++
		//		nBytes += int64(len(buf))
		//buf = nil
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	log.Println("Bytes:", nBytes, "Chunks:", nChunks)
}
