package chunks

import (
	"bufio"
	"io"
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

func Scanner(f *os.File) {
	nBytes, nChunks := int64(0), int64(0)
	scanner := bufio.NewScanner(f)
	buf := make([]byte, 0, 1<<20)
	scanner.Buffer(buf, 10<<20)
	scanner.Split(Chunks)
	for scanner.Scan() {
		nBytes += int64(len(scanner.Bytes()))
		nChunks++
		if nBytes >= 1073741824 {
			return
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
}

func Reader(f *os.File) {
	nBytes, nChunks := int64(0), int64(0)
	r := bufio.NewReader(f)
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
		if err != nil && err != io.EOF {
			log.Fatal(err)
		}
		if nBytes >= 1073741824 {
			return
		}
	}
}
