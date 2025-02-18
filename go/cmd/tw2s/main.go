package main

import (
	"github.com/fhluo/hanconv/go/internal/cmd/tw2s"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	tw2s.Execute()
}
