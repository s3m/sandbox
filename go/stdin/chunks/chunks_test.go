package chunks_test

import (
	"log"
	"os"
	"testing"

	"github.com/s3m/sandbox/go/stdin/chunks"
)

func BenchmarkScanner(b *testing.B) {
	f, err := os.OpenFile("../../../dataset/wine.json", os.O_RDONLY, os.ModePerm)
	if err != nil {
		log.Fatalf("open file error: %v", err)
		return
	}
	defer f.Close()
	chunks.Scanner(f, b)
}

func BenchmarkReader(b *testing.B) {
	f, err := os.OpenFile("../../../dataset/wine.json", os.O_RDONLY, os.ModePerm)
	if err != nil {
		log.Fatalf("open file error: %v", err)
		return
	}
	defer f.Close()
	chunks.Reader(f, b)
}
