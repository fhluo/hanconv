package main

import (
	"github.com/fhluo/hanconv/go/internal/cmd/jp2t"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	jp2t.Execute()
}
