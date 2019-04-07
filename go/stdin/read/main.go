package main

import (
	"bufio"
	"io"
	"log"
	"os"
	"time"
)

func main() {
	start := time.Now()
	nBytes, nChunks := int64(0), int64(0)
	r := bufio.NewReader(os.Stdin)
	buf := make([]byte, 0, 1024)
	for {
		n, err := r.Read(buf[:cap(buf)])
		buf = buf[:n]
		if n == 0 {
			if err == nil {
				continue
			}
			if err == io.EOF {
				break
			}
			log.Fatal(err)
		}
		nChunks++
		nBytes += int64(len(buf))
		// process buf
		if err != nil && err != io.EOF {
			log.Fatal(err)
		}
	}
	elapsed := time.Since(start)
	log.Printf("Bytes: %d Chunks: %d: epapsed: %v\n", nBytes, nChunks, elapsed)
}
