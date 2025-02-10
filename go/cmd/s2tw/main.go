package main

import (
	"github.com/fhluo/hanconv/go/pkg/cmd/s2tw"
	"log"
)

func init() {
	log.SetFlags(0)
}

func main() {
	s2tw.Execute()
}
