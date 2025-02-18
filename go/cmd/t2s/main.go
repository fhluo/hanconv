package main

import (
	"github.com/fhluo/hanconv/go/internal/cmd/t2s"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	t2s.Execute()
}
