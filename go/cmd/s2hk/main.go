package main

import (
	"github.com/fhluo/hanconv/go/internal/cmd/s2hk"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	s2hk.Execute()
}
