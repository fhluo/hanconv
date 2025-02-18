package main

import (
	"github.com/fhluo/hanconv/go/internal/cmd/s2t"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	s2t.Execute()
}
