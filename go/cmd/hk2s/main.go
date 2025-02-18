package main

import (
	"github.com/fhluo/hanconv/go/internal/cmd/hk2s"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	hk2s.Execute()
}
