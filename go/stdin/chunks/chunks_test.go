package chunks_test

import (
	"fmt"
	"log"
	"os"
	"runtime"
	"testing"

	"github.com/s3m/sandbox/go/stdin/chunks"
)

func BenchmarkScanner(b *testing.B) {
	PrintMemUsage()
	f, err := os.OpenFile("/dev/urandom", os.O_RDONLY, os.ModePerm)
	if err != nil {
		log.Fatalf("open file error: %v", err)
		return
	}
	defer f.Close()
	chunks.Scanner(f)
	PrintMemUsage()
	runtime.GC()
	PrintMemUsage()
}

func BenchmarkReader(b *testing.B) {
	PrintMemUsage()
	f, err := os.OpenFile("/dev/urandom", os.O_RDONLY, os.ModePerm)
	if err != nil {
		log.Fatalf("open file error: %v", err)
		return
	}
	defer f.Close()
	chunks.Reader(f)
	PrintMemUsage()
	runtime.GC()
	PrintMemUsage()
}

func PrintMemUsage() {
	var m runtime.MemStats
	runtime.ReadMemStats(&m)
	// For info on each, see: https://golang.org/pkg/runtime/#MemStats
	fmt.Printf("Alloc = %v MiB", bToMb(m.Alloc))
	fmt.Printf("\tTotalAlloc = %v MiB", bToMb(m.TotalAlloc))
	fmt.Printf("\tSys = %v MiB", bToMb(m.Sys))
	fmt.Printf("\tNumGC = %v\n", m.NumGC)
}

func bToMb(b uint64) uint64 {
	return b / 1024 / 1024
}
